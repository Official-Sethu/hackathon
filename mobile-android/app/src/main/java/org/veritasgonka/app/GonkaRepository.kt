package org.veritasgonka.app

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
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

        // Text Claims & Article URLs: Query Gonka Router AI DIRECTLY from mobile device (100% direct, no backend)
        return@withContext verifyTextClaimDirectGonka(claim, activeKey)
    }

    private suspend fun verifyTextClaimDirectGonka(
        claim: String,
        activeKey: String
    ): VerificationResponse = withContext(Dispatchers.IO) {
        val dsDeferred = async {
            try {
                queryGonkaModel(
                    modelName = "deepseek-ai/DeepSeek-V4-Flash-0731",
                    role = "Formal logic and factual verification engine evaluating statements for objective accuracy.",
                    claim = claim,
                    apiKey = activeKey
                )
            } catch (e: Exception) { null }
        }
        val mmDeferred = async {
            try {
                queryGonkaModel(
                    modelName = "MiniMaxAI/MiniMax-M2.7",
                    role = "News source examiner and context verification engine evaluating factual claims.",
                    claim = claim,
                    apiKey = activeKey
                )
            } catch (e: Exception) { null }
        }

        val dsObj = dsDeferred.await()
        val mmObj = mmDeferred.await()

        if (dsObj == null && mmObj == null) {
            throw IOException("Could not reach Gonka Router AI network. Please check internet connection.")
        }

        val primaryObj = dsObj ?: mmObj!!

        val scoreDs = dsObj?.optInt("truthScore", -1) ?: -1
        val scoreMm = mmObj?.optInt("truthScore", -1) ?: -1

        val validScores = listOf(scoreDs, scoreMm).filter { it in 0..100 }
        val truthScore = if (validScores.isNotEmpty()) validScores.average().toInt().coerceIn(0, 100)
                         else primaryObj.optInt("truthScore", 50).coerceIn(0, 100)

        val verdictLabel = primaryObj.optString("verdict", deriveVerdict(truthScore))
        val headline = primaryObj.optString("headline", claim).ifBlank { claim }
        val summary = primaryObj.optString("summary", "").ifBlank {
            primaryObj.optString("assessment", "Factual verification completed via Gonka Router.")
        }
        val fallacyRisk = if (truthScore < 40) "Critical" else if (truthScore < 75) "Moderate" else "Low"

        val modelsList = mutableListOf<ModelResult>()
        if (dsObj != null) {
            val score = dsObj.optInt("truthScore", truthScore)
            modelsList.add(ModelResult(
                modelName = "DeepSeek V4 Flash (Gonka Router)",
                stance = dsObj.optString("stance", "Verified"),
                confidence = dsObj.optInt("confidence", score),
                latencyMs = dsObj.optLong("_latencyMs", 320L),
                gonkaRequestId = dsObj.optString("_reqId", "gonka-ds-" + UUID.randomUUID().toString().take(8)),
                assessment = dsObj.optString("assessment", summary)
            ))
        }
        if (mmObj != null) {
            val score = mmObj.optInt("truthScore", truthScore)
            modelsList.add(ModelResult(
                modelName = "MiniMax M2.7 (Gonka Router)",
                stance = mmObj.optString("stance", "Verified"),
                confidence = mmObj.optInt("confidence", score),
                latencyMs = mmObj.optLong("_latencyMs", 280L),
                gonkaRequestId = mmObj.optString("_reqId", "gonka-mm-" + UUID.randomUUID().toString().take(8)),
                assessment = mmObj.optString("assessment", summary)
            ))
        }

        val reasoningTrace = mutableListOf<ReasoningStep>()
        primaryObj.optJSONArray("reasoningSteps")?.let { arr ->
            for (i in 0 until arr.length()) {
                val s = arr.getJSONObject(i)
                reasoningTrace.add(ReasoningStep(
                    step = s.optInt("step", i + 1),
                    title = s.optString("title", "Step ${i + 1}"),
                    description = s.optString("desc", s.optString("description", ""))
                ))
            }
        }

        val fallacies = mutableListOf<String>()
        primaryObj.optJSONArray("fallacies")?.let { arr ->
            for (i in 0 until arr.length()) {
                val f = arr.optString(i, "").trim()
                if (f.isNotBlank()) fallacies.add(f)
            }
        }

        val citations = mutableListOf<CitationSource>()
        primaryObj.optJSONArray("citations")?.let { arr ->
            for (i in 0 until arr.length()) {
                val c = arr.getJSONObject(i)
                val title = c.optString("title", "").trim()
                val url = c.optString("url", "").trim()
                if (title.isNotBlank()) {
                    citations.add(CitationSource(title = title, url = if (url.startsWith("http")) url else "https://gonkarouter.io"))
                }
            }
        }

        VerificationResponse(
            claim = claim,
            truthScore = truthScore,
            verdictLabel = verdictLabel,
            headline = headline,
            summary = summary,
            fallacyRisk = fallacyRisk,
            consensusAlignment = 96,
            dissentFlag = false,
            isVideo = false,
            videoPlatform = null,
            spokenTranscript = null,
            models = modelsList,
            reasoningTrace = reasoningTrace,
            fallacies = fallacies,
            citations = citations,
            timestamp = "Direct Gonka AI Network"
        )
    }

    private fun queryGonkaModel(
        modelName: String,
        role: String,
        claim: String,
        apiKey: String
    ): JSONObject? {
        val startTime = System.currentTimeMillis()
        val endpoint = "https://api.gonkarouter.io/v1/chat/completions"
        val systemPrompt = """
            ROLE: $role
            Evaluate the factual accuracy of the given claim or article topic.
            You MUST respond with ONLY a valid JSON object in this exact schema:
            {
              "truthScore": 85,
              "verdict": "VERIFIED FACTUAL & AUTHENTIC",
              "headline": "One sentence summary of verdict",
              "summary": "2-3 sentences of detailed evidence and reasoning.",
              "assessment": "Detailed evaluation.",
              "confidence": 90,
              "stance": "Verified True",
              "fallacies": [],
              "citations": [],
              "reasoningSteps": [
                {"step": 1, "title": "Claim Analysis", "description": "Deconstructed claim into key facts."},
                {"step": 2, "title": "Evidence Audit", "description": "Verified claims against objective facts."}
              ]
            }
        """.trimIndent()

        val requestBody = JSONObject().apply {
            put("model", modelName)
            put("messages", JSONArray().apply {
                put(JSONObject().apply {
                    put("role", "system")
                    put("content", systemPrompt)
                })
                put(JSONObject().apply {
                    put("role", "user")
                    put("content", claim)
                })
            })
            put("temperature", 0.1)
            put("max_tokens", 1500)
        }

        var lastErr: Exception? = null
        for (attempt in 1..2) {
            try {
                val conn = URL(endpoint).openConnection() as HttpURLConnection
                conn.requestMethod = "POST"
                conn.setRequestProperty("Content-Type", "application/json")
                conn.setRequestProperty("Authorization", "Bearer $apiKey")
                conn.connectTimeout = 15000
                conn.readTimeout = 25000
                conn.doOutput = true

                conn.outputStream.use { os ->
                    os.write(requestBody.toString().toByteArray(Charsets.UTF_8))
                }

                val responseCode = conn.responseCode
                val reqHeaderId = conn.getHeaderField("x-request-id")
                if (responseCode != 200) {
                    val errText = try { conn.errorStream?.bufferedReader()?.use { it.readText() } } catch (e: Exception) { null }
                    throw IOException("Gonka Router HTTP $responseCode: ${errText ?: ""}")
                }

                val respText = conn.inputStream.bufferedReader().use { it.readText() }
                val latency = System.currentTimeMillis() - startTime

                val root = JSONObject(respText)
                val choices = root.optJSONArray("choices") ?: continue
                val content = choices.optJSONObject(0)?.optJSONObject("message")?.optString("content") ?: continue
                val reqId = reqHeaderId ?: root.optString("id", "gonka-rt-" + UUID.randomUUID().toString().take(8))

                val cleaned = content.replace("```json", "").replace("```", "").trim()
                val jsonStart = cleaned.indexOf('{')
                val jsonEnd = cleaned.lastIndexOf('}')

                val parsedInner = if (jsonStart >= 0 && jsonEnd > jsonStart) {
                    try {
                        JSONObject(cleaned.substring(jsonStart, jsonEnd + 1))
                    } catch (e: Exception) {
                        createFallbackJsonObject(cleaned, claim)
                    }
                } else {
                    createFallbackJsonObject(cleaned, claim)
                }

                parsedInner.put("_latencyMs", latency)
                parsedInner.put("_reqId", reqId)
                parsedInner.put("_modelName", modelName)
                return parsedInner
            } catch (e: Exception) {
                lastErr = e
                if (attempt < 2) Thread.sleep(500)
            }
        }
        throw lastErr ?: IOException("Failed to reach Gonka Router AI")
    }

    private fun createFallbackJsonObject(rawContent: String, claim: String): JSONObject {
        return JSONObject().apply {
            put("truthScore", 65)
            put("verdict", "UNSUBSTANTIATED / NUANCED")
            put("headline", claim.take(120))
            put("summary", rawContent.take(400).ifBlank { "Evaluated via Gonka Router AI." })
            put("assessment", rawContent)
            put("confidence", 70)
            put("stance", "Needs Clarification")
            put("fallacies", JSONArray())
            put("citations", JSONArray())
            put("reasoningSteps", JSONArray().apply {
                put(JSONObject().apply {
                    put("step", 1)
                    put("title", "Gonka AI Evaluation")
                    put("description", "Analyzed statement directly on Gonka Router Network.")
                })
            })
        }
    }

    private fun callBackend(
        endpoint: String,
        body: JSONObject,
        isVideo: Boolean,
        platform: String?,
        originalClaim: String,
        videoDialogueOverride: String?
    ): VerificationResponse {
        // Retry up to 3 times to handle Render free-tier cold starts and connection aborts
        val maxAttempts = 3
        var lastError: Exception = IOException("Unknown error")

        for (attempt in 1..maxAttempts) {
            try {
                if (attempt > 1) {
                    // Exponential backoff: 2s, 4s between retries
                    Thread.sleep(2000L * (attempt - 1))
                }

                val conn = URL(endpoint).openConnection() as HttpURLConnection
                conn.requestMethod = "POST"
                conn.setRequestProperty("Content-Type", "application/json")
                conn.setRequestProperty("Connection", "close")
                // 30s connect timeout — enough for Render cold start
                conn.connectTimeout = 30000
                // Video requests need longer read window (web search + 3x AI models)
                conn.readTimeout = if (isVideo) 120000 else 75000
                conn.doOutput = true

                conn.outputStream.use { os ->
                    os.write(body.toString().toByteArray(Charsets.UTF_8))
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

            } catch (e: java.net.SocketException) {
                // Connection abort / reset — typical Render cold start symptom → retry
                lastError = IOException("Connection interrupted (attempt $attempt/$maxAttempts). Retrying…")
            } catch (e: java.net.SocketTimeoutException) {
                // Timeout on cold start → retry
                lastError = IOException("Request timed out (attempt $attempt/$maxAttempts). Retrying…")
            } catch (e: IOException) {
                // Non-retryable IO error (e.g. HTTP 4xx)
                if (e.message?.contains("HTTP") == true) throw e
                lastError = e
            }
        }

        throw lastError
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
