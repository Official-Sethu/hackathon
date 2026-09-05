package org.veritasgonka.app

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject
import java.io.IOException
import java.net.HttpURLConnection
import java.net.URL
import java.util.UUID

data class CitationSource(
    val title: String,
    val url: String
)

data class ReasoningStep(
    val step: Int,
    val title: String,
    val description: String
)

data class ModelResult(
    val modelName: String,
    val stance: String,
    val confidence: Int,
    val latencyMs: Long,
    val gonkaRequestId: String,
    val assessment: String
)

data class VerificationResponse(
    val claim: String,
    val truthScore: Int,
    val verdictLabel: String,
    val headline: String,
    val summary: String,
    val fallacyRisk: String,
    val consensusAlignment: Int = 95,
    val dissentFlag: Boolean = false,
    val isVideo: Boolean = false,
    val videoPlatform: String? = null,
    val spokenTranscript: String? = null,
    val models: List<ModelResult> = emptyList(),
    val reasoningTrace: List<ReasoningStep> = emptyList(),
    val fallacies: List<String> = emptyList(),
    val citations: List<CitationSource> = emptyList(),
    val timestamp: String = ""
)

class GonkaRepository {
    // All verification — both text claims and video links — routes through the Trace backend.
    // The backend runs web search + 3-model ensemble + weighted consensus before returning.
    private val backendBase = "https://trace-backend-7bbm.onrender.com"
    private val defaultApiKey = "sk-PxMSYFiyuDP14zSxvfyBNUpqwIP46ARYjyJr2RCnBtn15Dxd"

    fun isShortVideoUrl(urlStr: String): Boolean {
        val lower = urlStr.trim().lowercase()
        return lower.contains("vt.tiktok.com") ||
               lower.contains("tiktok.com") ||
               lower.contains("vm.tiktok.com") ||
               lower.contains("facebook.com/reel") ||
               lower.contains("fb.watch") ||
               lower.contains("facebook.com/watch") ||
               lower.contains("fb.com/reel") ||
               lower.contains("youtube.com/shorts") ||
               lower.contains("youtu.be") ||
               lower.contains("instagram.com/reel") ||
               lower.contains("/shorts/") ||
               ((lower.contains("x.com") || lower.contains("twitter.com")) && lower.contains("/status"))
    }

    fun detectVideoPlatform(urlStr: String): String {
        val lower = urlStr.lowercase()
        return when {
            lower.contains("tiktok.com") -> "TikTok"
            lower.contains("facebook.com") || lower.contains("fb.watch") || lower.contains("fb.com") -> "Facebook"
            lower.contains("youtube.com/shorts") || lower.contains("youtu.be") -> "YouTube Shorts"
            lower.contains("instagram.com/reel") -> "Instagram Reels"
            lower.contains("x.com") || lower.contains("twitter.com") -> "X Video"
            else -> "Short Video"
        }
    }

    suspend fun verifyClaim(
        claim: String,
        apiKey: String? = null,
        videoDialogueOverride: String? = null
    ): VerificationResponse = withContext(Dispatchers.IO) {

        val isVideo = isShortVideoUrl(claim)
        val platform = if (isVideo) detectVideoPlatform(claim) else null
        val activeKey = if (!apiKey.isNullOrBlank()) apiKey.trim() else defaultApiKey

        if (isVideo) {
            // Short Video Reels: POST to /api/verify_video
            // Backend handles: redirect follow, metadata extraction, web search, 3-model ensemble
            return@withContext callBackend(
                endpoint = "$backendBase/api/verify_video",
                body = JSONObject().apply {
                    put("video_url", claim)
                    if (!videoDialogueOverride.isNullOrBlank()) {
                        put("spoken_transcript", videoDialogueOverride.trim())
                    }
                    if (!apiKey.isNullOrBlank()) {
                        put("api_key", apiKey.trim())
                    }
                },
                isVideo = true,
                platform = platform,
                originalClaim = claim,
                videoDialogueOverride = videoDialogueOverride
            )
        }

        // Text Claims & Article URLs: POST to /api/verify
        // Backend runs web search + chain-of-thought + 3-model weighted consensus
        return@withContext callBackend(
            endpoint = "$backendBase/api/verify",
            body = JSONObject().apply {
                put("claim", claim)
                if (!apiKey.isNullOrBlank()) {
                    put("api_key", apiKey.trim())
                }
            },
            isVideo = false,
            platform = null,
            originalClaim = claim,
            videoDialogueOverride = null
        )
    }

    private fun callBackend(
        endpoint: String,
        body: JSONObject,
        isVideo: Boolean,
        platform: String?,
        originalClaim: String,
        videoDialogueOverride: String?
    ): VerificationResponse {
        val conn = URL(endpoint).openConnection() as HttpURLConnection
        conn.requestMethod = "POST"
        conn.setRequestProperty("Content-Type", "application/json")
        conn.connectTimeout = 20000
        // Video requests spool up the server — wait up to 90s with no fallback
        conn.readTimeout = if (isVideo) 90000 else 60000
        conn.doOutput = true

        try {
            conn.outputStream.use { os ->
                os.write(body.toString().toByteArray(Charsets.UTF_8))
            }
        } catch (e: Exception) {
            throw IOException("Network connection to Trace backend failed: ${e.message}")
        }

        val responseCode = conn.responseCode
        if (responseCode != 200) {
            val errMsg = try {
                conn.errorStream?.bufferedReader()?.use { it.readText() }
            } catch (e: Exception) { null }
            throw IOException("Trace backend error (HTTP $responseCode): ${errMsg ?: "Server error"}")
        }

        val respText = conn.inputStream.bufferedReader().use { it.readText() }
        return parseBackendResponse(respText, isVideo, platform, originalClaim, videoDialogueOverride)
    }

    private fun parseBackendResponse(
        json: String,
        isVideo: Boolean,
        platform: String?,
        originalClaim: String,
        videoDialogueOverride: String?
    ): VerificationResponse {
        val root = JSONObject(json)

        val truthScore = root.optInt("truthScore", 50).coerceIn(0, 100)
        val verdictLabel = root.optString("verdictLabel", deriveVerdict(truthScore))
        val headline = root.optString("headline", originalClaim).ifBlank { originalClaim }
        val summary = root.optString("summary", "").ifBlank { "Verification complete." }
        val fallacyRisk = root.optString("fallacyRisk",
            if (truthScore < 40) "Critical" else if (truthScore < 75) "Moderate" else "Low"
        )
        val consensusAlignment = root.optInt("consensusAlignment", 95)
        val dissentFlag = root.optBoolean("dissentFlag", false)
        val timestamp = root.optString("timestamp", "")
        val spokenTranscript = videoDialogueOverride
            ?: root.optString("spokenTranscript", null)

        // Models array
        val modelsList = mutableListOf<ModelResult>()
        root.optJSONArray("models")?.let { arr ->
            for (i in 0 until arr.length()) {
                val m = arr.getJSONObject(i)
                modelsList.add(ModelResult(
                    modelName = m.optString("model", m.optString("modelName", "AI Model")),
                    stance = m.optString("stance", "Evaluated"),
                    confidence = m.optInt("confidence", truthScore),
                    latencyMs = m.optLong("latencyMs", m.optLong("latency_ms", 0L)),
                    gonkaRequestId = m.optString("gonkaRequestId",
                        m.optString("gonka_request_id", "gonka-" + UUID.randomUUID().toString().take(8))),
                    assessment = m.optString("assessment", summary)
                ))
            }
        }

        // Reasoning trace
        val reasoningTrace = mutableListOf<ReasoningStep>()
        root.optJSONArray("reasoningTrace")?.let { arr ->
            for (i in 0 until arr.length()) {
                val s = arr.getJSONObject(i)
                reasoningTrace.add(ReasoningStep(
                    step = s.optInt("step", i + 1),
                    title = s.optString("title", "Step ${i + 1}"),
                    description = s.optString("description", "")
                ))
            }
        }

        // Fallacies
        val fallacies = mutableListOf<String>()
        root.optJSONArray("fallacies")?.let { arr ->
            for (i in 0 until arr.length()) {
                val f = arr.optString(i, "").trim()
                if (f.isNotBlank()) fallacies.add(f)
            }
        }

        // Citations
        val citations = mutableListOf<CitationSource>()
        root.optJSONArray("citations")?.let { arr ->
            for (i in 0 until arr.length()) {
                val c = arr.getJSONObject(i)
                val title = c.optString("title", "").trim()
                val url = c.optString("url", "").trim()
                if (title.isNotBlank() && url.isNotBlank()) {
                    citations.add(CitationSource(title = title, url = url))
                }
            }
        }

        return VerificationResponse(
            claim = originalClaim,
            truthScore = truthScore,
            verdictLabel = verdictLabel,
            headline = headline,
            summary = summary,
            fallacyRisk = fallacyRisk,
            consensusAlignment = consensusAlignment,
            dissentFlag = dissentFlag,
            isVideo = isVideo,
            videoPlatform = platform,
            spokenTranscript = spokenTranscript,
            models = modelsList,
            reasoningTrace = reasoningTrace,
            fallacies = fallacies,
            citations = citations,
            timestamp = timestamp
        )
    }

    private fun deriveVerdict(score: Int) = when {
        score >= 75 -> "VERIFIED FACTUAL & AUTHENTIC"
        score >= 35 -> "UNSUBSTANTIATED / NUANCED"
        else -> "FABRICATED OR DEBUNKED"
    }
}
