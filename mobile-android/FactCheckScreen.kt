package org.veritasgonka.app

import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.tween
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

// Design tokens
private val BgPage        = Color(0xFFF3F4F6)
private val BgCard        = Color(0xFFFFFFFF)
private val BgSubtle      = Color(0xFFF8F9FA)
private val BorderLight   = Color(0xFFE5E7EB)
private val TextPrimary   = Color(0xFF111827)
private val TextSecondary = Color(0xFF4B5563)
private val TextMuted     = Color(0xFF6B7280)
private val AccentIndigo  = Color(0xFF6366F1)
private val AccentViolet  = Color(0xFF8B5CF6)
private val ColorVerified = Color(0xFF059669)
private val ColorDebunked = Color(0xFFDC2626)
private val ColorPending  = Color(0xFFD97706)
private val ColorIndigoBg = Color(0xFFEEF2FF)
private val ColorGreenBg  = Color(0xFFECFDF5)
private val ColorRedBg    = Color(0xFFFEF2F2)

@Composable
fun FactCheckScreen(repository: GonkaRepository = remember { GonkaRepository() }) {
    var activeTab by remember { mutableStateOf(0) } // 0=Text, 1=News Link, 2=Short Video
    var claimInput by remember { mutableStateOf("") }
    var urlInput by remember { mutableStateOf("") }
    var videoUrlInput by remember { mutableStateOf("") }
    var isLoading by remember { mutableStateOf(false) }
    var cooldownSeconds by remember { mutableStateOf(0) }
    var errorMessage by remember { mutableStateOf<String?>(null) }
    var result by remember { mutableStateOf<VerificationResponse?>(null) }
    val coroutineScope = rememberCoroutineScope()
    val scrollState = rememberScrollState()

    fun getActiveInput(): String = when (activeTab) {
        1 -> urlInput
        2 -> videoUrlInput
        else -> claimInput
    }

    fun runVerification() {
        val input = getActiveInput().trim()
        if (input.isBlank() || cooldownSeconds > 0 || isLoading) return
        isLoading = true
        errorMessage = null
        coroutineScope.launch {
            try {
                result = repository.verifyClaim(input)
            } catch (e: Exception) {
                errorMessage = "Rate limit or network issue. Please wait a moment and try again."
            } finally {
                isLoading = false
                cooldownSeconds = 5
                while (cooldownSeconds > 0) {
                    delay(1000)
                    cooldownSeconds -= 1
                }
            }
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(BgPage)
            .verticalScroll(scrollState),
        verticalArrangement = Arrangement.spacedBy(0.dp)
    ) {
        // Top App Bar
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .background(BgCard)
                .padding(horizontal = 20.dp, vertical = 16.dp)
        ) {
            Text(
                text = "VeritasGonka",
                fontWeight = FontWeight.ExtraBold,
                fontSize = 20.sp,
                color = TextPrimary
            )
        }
        HorizontalDivider(color = BorderLight)

        // Content
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 20.dp),
            verticalArrangement = Arrangement.spacedBy(20.dp)
        ) {
            Spacer(modifier = Modifier.height(4.dp))

            // Hero headline
            Text(
                text = "Is that headline actually true?",
                fontSize = 22.sp,
                fontWeight = FontWeight.Bold,
                color = TextPrimary,
                lineHeight = 30.sp
            )
            Text(
                text = "Three independent AI systems check it and compare answers to give you one honest verdict.",
                fontSize = 14.sp,
                color = TextSecondary,
                lineHeight = 20.sp
            )

            // Input card
            Card(
                colors = CardDefaults.cardColors(containerColor = BgCard),
                shape = RoundedCornerShape(16.dp),
                elevation = CardDefaults.cardElevation(defaultElevation = 1.dp),
                modifier = Modifier.fillMaxWidth()
            ) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(14.dp)
                ) {
                    // Segmented tab row
                    SegmentedTabRow(
                        tabs = listOf("Text", "News Link", "Short Video"),
                        selectedIndex = activeTab,
                        onTabSelected = { activeTab = it }
                    )

                    // Input fields per tab
                    when (activeTab) {
                        0 -> OutlinedTextField(
                            value = claimInput,
                            onValueChange = { claimInput = it },
                            placeholder = {
                                Text(
                                    "Paste any claim or headline...",
                                    color = TextMuted,
                                    fontSize = 14.sp
                                )
                            },
                            modifier = Modifier.fillMaxWidth().height(110.dp),
                            colors = OutlinedTextFieldDefaults.colors(
                                focusedBorderColor = AccentIndigo,
                                unfocusedBorderColor = BorderLight
                            ),
                            shape = RoundedCornerShape(10.dp)
                        )
                        1 -> OutlinedTextField(
                            value = urlInput,
                            onValueChange = { urlInput = it },
                            placeholder = {
                                Text(
                                    "https://bbc.com/news/article...",
                                    color = TextMuted,
                                    fontSize = 14.sp
                                )
                            },
                            modifier = Modifier.fillMaxWidth(),
                            colors = OutlinedTextFieldDefaults.colors(
                                focusedBorderColor = AccentIndigo,
                                unfocusedBorderColor = BorderLight
                            ),
                            shape = RoundedCornerShape(10.dp),
                            singleLine = true
                        )
                        2 -> {
                            // Platform chips
                            Row(
                                horizontalArrangement = Arrangement.spacedBy(8.dp),
                                modifier = Modifier.horizontalScroll(rememberScrollState())
                            ) {
                                PlatformChip("TikTok", Color(0xFF010101), Color(0xFFF0F0F0))
                                PlatformChip("YouTube Shorts", Color(0xFFFF0000), Color(0xFFFFF0F0))
                                PlatformChip("Instagram Reels", Color(0xFFC13584), Color(0xFFFDF0F8))
                                PlatformChip("X Video", Color(0xFF000000), Color(0xFFF0F0F0))
                            }
                            OutlinedTextField(
                                value = videoUrlInput,
                                onValueChange = { videoUrlInput = it },
                                placeholder = {
                                    Text(
                                        "Paste TikTok, YouTube Short, or Reel link...",
                                        color = TextMuted,
                                        fontSize = 14.sp
                                    )
                                },
                                modifier = Modifier.fillMaxWidth(),
                                colors = OutlinedTextFieldDefaults.colors(
                                    focusedBorderColor = AccentIndigo,
                                    unfocusedBorderColor = BorderLight
                                ),
                                shape = RoundedCornerShape(10.dp),
                                singleLine = true
                            )
                            Text(
                                text = "We extract spoken dialogue and captions from the video audio to check what is actually said.",
                                fontSize = 12.sp,
                                color = TextMuted,
                                lineHeight = 17.sp
                            )
                        }
                    }

                    // Quick examples
                    Text(
                        text = "TRY A QUICK EXAMPLE",
                        fontSize = 10.sp,
                        fontWeight = FontWeight.Bold,
                        color = TextMuted,
                        letterSpacing = 0.6.sp
                    )
                    Row(
                        horizontalArrangement = Arrangement.spacedBy(8.dp),
                        modifier = Modifier.horizontalScroll(rememberScrollState())
                    ) {
                        ExampleChip(label = "Health Myth", isDebunked = true) {
                            activeTab = 0
                            claimInput = "WHO announces coffee cures Covid-19 mutations"
                        }
                        ExampleChip(label = "TikTok Reel", isDebunked = true) {
                            activeTab = 2
                            videoUrlInput = "TikTok Reel: Baking soda in warm water dissolves kidney stones in 24 hours"
                        }
                        ExampleChip(label = "YT Short", isDebunked = false) {
                            activeTab = 2
                            videoUrlInput = "YouTube Short: Overnight smartphone charging causes battery explosion"
                        }
                        ExampleChip(label = "Space", isDebunked = false) {
                            activeTab = 0
                            claimInput = "James Webb Space Telescope detects atmospheric water vapor on exoplanet K2-18b"
                        }
                    }

                    errorMessage?.let { err ->
                        Card(
                            colors = CardDefaults.cardColors(containerColor = ColorRedBg),
                            border = BorderStroke(1.dp, ColorDebunked.copy(alpha = 0.3f)),
                            shape = RoundedCornerShape(12.dp),
                            modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp)
                        ) {
                            Column(
                                modifier = Modifier.padding(14.dp),
                                verticalArrangement = Arrangement.spacedBy(4.dp)
                            ) {
                                Text(
                                    text = "Oops, AI Network Unreachable",
                                    fontWeight = FontWeight.Bold,
                                    fontSize = 14.sp,
                                    color = ColorDebunked
                                )
                                Text(
                                    text = err,
                                    fontSize = 12.sp,
                                    color = TextSecondary,
                                    lineHeight = 16.sp
                                )
                            }
                        }
                    }

                    // CTA button
                    Box(
                        modifier = Modifier
                            .fillMaxWidth()
                            .height(52.dp)
                            .clip(RoundedCornerShape(12.dp))
                            .background(
                                if (cooldownSeconds > 0 || isLoading)
                                    Brush.horizontalGradient(listOf(Color(0xFF9CA3AF), Color(0xFF9CA3AF)))
                                else
                                    Brush.horizontalGradient(listOf(AccentIndigo, AccentViolet))
                            )
                            .clickable(enabled = !isLoading && cooldownSeconds == 0 && getActiveInput().isNotBlank()) {
                                runVerification()
                            },
                        contentAlignment = Alignment.Center
                    ) {
                        if (isLoading) {
                            CircularProgressIndicator(
                                color = Color.White,
                                modifier = Modifier.size(22.dp),
                                strokeWidth = 2.dp
                            )
                        } else if (cooldownSeconds > 0) {
                            Text(
                                text = "Cooldown (${cooldownSeconds}s)...",
                                color = Color.White,
                                fontWeight = FontWeight.Bold,
                                fontSize = 15.sp
                            )
                        } else {
                            Text(
                                text = "Check This Claim",
                                color = Color.White,
                                fontWeight = FontWeight.Bold,
                                fontSize = 15.sp
                            )
                        }
                    }
                }
            }

            // Results section
            result?.let { res ->
                ResultsCard(res)
            }

            Spacer(modifier = Modifier.height(24.dp))
        }
    }
}

@Composable
private fun SegmentedTabRow(
    tabs: List<String>,
    selectedIndex: Int,
    onTabSelected: (Int) -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(10.dp))
            .background(Color(0xFFF3F4F6)),
        horizontalArrangement = Arrangement.spacedBy(0.dp)
    ) {
        tabs.forEachIndexed { index, label ->
            val isSelected = index == selectedIndex
            Box(
                modifier = Modifier
                    .weight(1f)
                    .clip(RoundedCornerShape(8.dp))
                    .background(
                        if (isSelected) AccentIndigo else Color.Transparent
                    )
                    .clickable { onTabSelected(index) }
                    .padding(vertical = 9.dp),
                contentAlignment = Alignment.Center
            ) {
                Text(
                    text = label,
                    fontSize = 12.sp,
                    fontWeight = if (isSelected) FontWeight.Bold else FontWeight.Medium,
                    color = if (isSelected) Color.White else TextMuted,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
            }
        }
    }
}

@Composable
private fun PlatformChip(label: String, iconColor: Color, bgColor: Color) {
    Box(
        modifier = Modifier
            .clip(RoundedCornerShape(8.dp))
            .background(bgColor)
            .border(1.dp, BorderLight, RoundedCornerShape(8.dp))
            .padding(horizontal = 10.dp, vertical = 6.dp)
    ) {
        Text(text = label, fontSize = 12.sp, fontWeight = FontWeight.Medium, color = iconColor)
    }
}

@Composable
private fun ExampleChip(label: String, isDebunked: Boolean, onClick: () -> Unit) {
    val dotColor = if (isDebunked) ColorDebunked else ColorVerified
    Row(
        modifier = Modifier
            .clip(RoundedCornerShape(20.dp))
            .background(BgCard)
            .border(1.dp, BorderLight, RoundedCornerShape(20.dp))
            .clickable { onClick() }
            .padding(horizontal = 12.dp, vertical = 7.dp),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(6.dp)
    ) {
        Box(
            modifier = Modifier
                .size(7.dp)
                .clip(CircleShape)
                .background(dotColor)
        )
        Text(text = label, fontSize = 12.sp, color = TextPrimary, fontWeight = FontWeight.Medium)
    }
}

@Composable
private fun ResultsCard(res: VerificationResponse) {
    val scoreColor = when {
        res.truthScore >= 80 -> ColorVerified
        res.truthScore >= 45 -> ColorPending
        else -> ColorDebunked
    }
    val scoreBg = when {
        res.truthScore >= 80 -> ColorGreenBg
        res.truthScore >= 45 -> Color(0xFFFFFBEB)
        else -> ColorRedBg
    }

    Card(
        colors = CardDefaults.cardColors(containerColor = BgCard),
        shape = RoundedCornerShape(16.dp),
        elevation = CardDefaults.cardElevation(defaultElevation = 1.dp),
        modifier = Modifier.fillMaxWidth()
    ) {
        Column(
            modifier = Modifier.padding(20.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            // Gauge + verdict
            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(16.dp)
            ) {
                // Circular gauge
                Box(contentAlignment = Alignment.Center, modifier = Modifier.size(88.dp)) {
                    androidx.compose.foundation.Canvas(modifier = Modifier.size(88.dp)) {
                        drawArc(
                            color = BorderLight,
                            startAngle = -90f,
                            sweepAngle = 360f,
                            useCenter = false,
                            style = Stroke(width = 7.dp.toPx(), cap = StrokeCap.Round)
                        )
                        drawArc(
                            color = scoreColor,
                            startAngle = -90f,
                            sweepAngle = 360f * res.truthScore / 100f,
                            useCenter = false,
                            style = Stroke(width = 7.dp.toPx(), cap = StrokeCap.Round)
                        )
                    }
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text(
                            text = "${res.truthScore}%",
                            fontSize = 20.sp,
                            fontWeight = FontWeight.Black,
                            color = scoreColor
                        )
                        Text(
                            text = "TRUTH",
                            fontSize = 8.sp,
                            fontWeight = FontWeight.Bold,
                            color = TextMuted
                        )
                    }
                }

                Column(verticalArrangement = Arrangement.spacedBy(6.dp)) {
                    // Verdict badge
                    Box(
                        modifier = Modifier
                            .clip(RoundedCornerShape(20.dp))
                            .background(scoreBg)
                            .padding(horizontal = 10.dp, vertical = 4.dp)
                    ) {
                        Text(
                            text = res.verdictLabel,
                            fontSize = 12.sp,
                            fontWeight = FontWeight.ExtraBold,
                            color = scoreColor
                        )
                    }
                    Text(
                        text = res.headline,
                        fontSize = 14.sp,
                        fontWeight = FontWeight.Bold,
                        color = TextPrimary,
                        lineHeight = 20.sp,
                        maxLines = 3,
                        overflow = TextOverflow.Ellipsis
                    )
                }
            }

            // Dissent flag warning
            if (res.dissentFlag) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clip(RoundedCornerShape(8.dp))
                        .background(Color(0xFFFFFBEB))
                        .border(1.dp, Color(0xFFFCD34D), RoundedCornerShape(8.dp))
                        .padding(10.dp),
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Box(
                        modifier = Modifier
                            .size(8.dp)
                            .clip(CircleShape)
                            .background(ColorPending)
                    )
                    Text(
                        text = "Models disagree significantly — treat this result with caution",
                        fontSize = 12.sp,
                        color = Color(0xFF92400E),
                        lineHeight = 16.sp
                    )
                }
            }

            // Video spoken audio card
            if (res.isVideo) {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clip(RoundedCornerShape(10.dp))
                        .background(ColorIndigoBg)
                        .border(1.dp, Color(0xFFC7D2FE), RoundedCornerShape(10.dp))
                        .padding(12.dp),
                    verticalArrangement = Arrangement.spacedBy(4.dp)
                ) {
                    Text(
                        text = "${res.videoPlatform ?: "Short Video"}: Spoken Audio",
                        fontSize = 11.sp,
                        fontWeight = FontWeight.Bold,
                        color = AccentIndigo
                    )
                    Text(
                        text = "\"${res.spokenTranscript ?: res.claim}\"",
                        fontSize = 13.sp,
                        color = TextPrimary,
                        lineHeight = 18.sp
                    )
                }
            }

            // Summary
            Text(
                text = res.summary,
                fontSize = 13.sp,
                color = TextSecondary,
                lineHeight = 19.sp
            )

            // Reasoning trace
            if (res.reasoningTrace.isNotEmpty()) {
                HorizontalDivider(color = BorderLight)
                Text(
                    text = "REASONING TRACE",
                    fontSize = 10.sp,
                    fontWeight = FontWeight.Bold,
                    color = TextMuted,
                    letterSpacing = 0.6.sp
                )
                res.reasoningTrace.forEach { step ->
                    Column(
                        modifier = Modifier
                            .fillMaxWidth()
                            .clip(RoundedCornerShape(8.dp))
                            .background(BgSubtle)
                            .padding(10.dp),
                        verticalArrangement = Arrangement.spacedBy(3.dp)
                    ) {
                        Text(
                            text = "${step.step}. ${step.title}",
                            fontSize = 12.sp,
                            fontWeight = FontWeight.Bold,
                            color = AccentIndigo
                        )
                        Text(
                            text = step.description,
                            fontSize = 12.sp,
                            color = TextSecondary,
                            lineHeight = 17.sp
                        )
                    }
                }
            }

            // Citations
            if (res.citations.isNotEmpty()) {
                HorizontalDivider(color = BorderLight)
                Text(
                    text = "SOURCES",
                    fontSize = 10.sp,
                    fontWeight = FontWeight.Bold,
                    color = TextMuted,
                    letterSpacing = 0.6.sp
                )
                res.citations.forEach { cite ->
                    Row(
                        horizontalArrangement = Arrangement.spacedBy(8.dp),
                        verticalAlignment = Alignment.Top,
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Box(
                            modifier = Modifier
                                .size(6.dp)
                                .clip(CircleShape)
                                .background(AccentIndigo)
                                .padding(top = 6.dp)
                        )
                        Column {
                            Text(
                                text = cite.title,
                                fontSize = 12.sp,
                                fontWeight = FontWeight.Medium,
                                color = AccentIndigo,
                                maxLines = 1,
                                overflow = TextOverflow.Ellipsis
                            )
                            Text(
                                text = cite.url,
                                fontSize = 11.sp,
                                color = TextMuted,
                                maxLines = 1,
                                overflow = TextOverflow.Ellipsis
                            )
                        }
                    }
                }
            }

            // Fallacies (only show if not the default "no fallacies" message)
            val realFallacies = res.fallacies.filter {
                !it.contains("No significant", ignoreCase = true)
            }
            if (realFallacies.isNotEmpty()) {
                HorizontalDivider(color = BorderLight)
                Text(
                    text = "FALLACIES DETECTED",
                    fontSize = 10.sp,
                    fontWeight = FontWeight.Bold,
                    color = ColorDebunked,
                    letterSpacing = 0.6.sp
                )
                realFallacies.forEach { fallacy ->
                    Row(
                        horizontalArrangement = Arrangement.spacedBy(8.dp),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Box(
                            modifier = Modifier
                                .size(6.dp)
                                .clip(CircleShape)
                                .background(ColorDebunked)
                        )
                        Text(
                            text = fallacy,
                            fontSize = 12.sp,
                            color = TextSecondary,
                            lineHeight = 16.sp
                        )
                    }
                }
            }

            // Consensus alignment bar
            if (res.consensusAlignment > 0) {
                HorizontalDivider(color = BorderLight)
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = "Model Consensus",
                        fontSize = 11.sp,
                        color = TextMuted,
                        fontWeight = FontWeight.Medium
                    )
                    Text(
                        text = "${res.consensusAlignment}%",
                        fontSize = 11.sp,
                        color = AccentIndigo,
                        fontWeight = FontWeight.Bold
                    )
                }
                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(4.dp)
                        .clip(RoundedCornerShape(2.dp))
                        .background(BorderLight)
                ) {
                    Box(
                        modifier = Modifier
                            .fillMaxWidth(res.consensusAlignment / 100f)
                            .height(4.dp)
                            .clip(RoundedCornerShape(2.dp))
                            .background(
                                Brush.horizontalGradient(
                                    listOf(AccentIndigo, AccentViolet)
                                )
                            )
                    )
                }
            }
        }
    }
}
