import SwiftUI

// MARK: - Design Tokens
private extension Color {
    static let bgPage        = Color(UIColor.systemGroupedBackground)
    static let bgCard        = Color(UIColor.systemBackground)
    static let bgSubtle      = Color(UIColor(red: 0.97, green: 0.97, blue: 0.98, alpha: 1))
    static let borderLight   = Color(UIColor.separator).opacity(0.4)
    static let textPrimary   = Color(UIColor.label)
    static let textSecondary = Color(UIColor.secondaryLabel)
    static let textMuted     = Color(UIColor.tertiaryLabel)
    static let accentIndigo  = Color(red: 0.388, green: 0.400, blue: 0.945) // #6366F1
    static let accentViolet  = Color(red: 0.545, green: 0.361, blue: 0.965) // #8B5CF6
    static let colorVerified = Color(red: 0.020, green: 0.588, blue: 0.412) // #059669
    static let colorDebunked = Color(red: 0.863, green: 0.149, blue: 0.149) // #DC2626
    static let colorPending  = Color(red: 0.851, green: 0.467, blue: 0.024) // #D97706
    static let colorIndigoBg = Color(red: 0.933, green: 0.937, blue: 1.000) // #EEF2FF
    static let colorGreenBg  = Color(red: 0.925, green: 0.992, blue: 0.961) // #ECFDF5
    static let colorRedBg    = Color(red: 0.996, green: 0.949, blue: 0.949) // #FEF2F2
}

// MARK: - Main View
struct FactCheckView: View {
    @State private var activeTab: Int = 0
    @State private var claimText: String = ""
    @State private var urlText: String = ""
    @State private var videoUrlText: String = ""
    @State private var videoDialogueOverride: String = ""   // manual input when metadata unavailable
    @State private var needsManualClaim: Bool = false       // triggers manual input UI
    @State private var isLoading: Bool = false
    @State private var result: VerificationResult? = nil
    @State private var cooldownSeconds: Int = 0
    @State private var errorMessage: String? = nil

    private var activeInput: String {
        switch activeTab {
        case 1: return urlText
        case 2: return videoUrlText
        default: return claimText
        }
    }

    var body: some View {
        NavigationView {
            ZStack {
                Color.bgPage.ignoresSafeArea()

                ScrollView {
                    VStack(alignment: .leading, spacing: 20) {

                        // Hero headline
                        VStack(alignment: .leading, spacing: 6) {
                            Text("Follow The Claim. Find The Truth.")
                                .font(.system(size: 22, weight: .bold))
                                .foregroundColor(.textPrimary)
                            Text("Three independent AI systems check it and compare answers to give you one honest verdict.")
                                .font(.system(size: 14))
                                .foregroundColor(.textSecondary)
                                .lineSpacing(2)
                        }

                        // Input card
                        VStack(alignment: .leading, spacing: 14) {

                            // Segmented tab row
                            SegmentedTabRow(
                                tabs: ["Text", "News Link", "Short Video"],
                                selectedIndex: $activeTab
                            )

                            // Input area per tab
                            Group {
                                switch activeTab {
                                case 1:
                                    StyledTextField(
                                        text: $urlText,
                                        placeholder: "https://bbc.com/news/article..."
                                    )
                                case 2:
                                    VStack(alignment: .leading, spacing: 10) {
                                        ScrollView(.horizontal, showsIndicators: false) {
                                            HStack(spacing: 8) {
                                                PlatformChip(label: "TikTok")
                                                PlatformChip(label: "YouTube Shorts")
                                                PlatformChip(label: "Instagram Reels")
                                                PlatformChip(label: "Facebook Reels")
                                                PlatformChip(label: "X Video")
                                            }
                                        }
                                        StyledTextField(
                                            text: Binding(
                                                get: { videoUrlText },
                                                set: {
                                                    videoUrlText = $0
                                                    needsManualClaim = false
                                                    videoDialogueOverride = ""
                                                }
                                            ),
                                            placeholder: "Paste TikTok, YouTube Short, Reel or Facebook link..."
                                        )
                                        // Manual claim prompt — appears when video metadata cannot be extracted
                                        if needsManualClaim {
                                            VStack(alignment: .leading, spacing: 8) {
                                                Label("We couldn't read this video's content", systemImage: "video.slash")
                                                    .font(.system(size: 14, weight: .bold))
                                                    .foregroundColor(Color(red: 0.573, green: 0.251, blue: 0.055))
                                                Text("Please type what the video claims so we can fact-check it accurately:")
                                                    .font(.system(size: 13))
                                                    .foregroundColor(Color(red: 0.471, green: 0.208, blue: 0.055))
                                                    .lineSpacing(2)
                                                TextEditor(text: $videoDialogueOverride)
                                                    .frame(height: 80)
                                                    .padding(10)
                                                    .background(Color(red: 1.0, green: 0.988, blue: 0.922))
                                                    .cornerRadius(10)
                                                    .overlay(
                                                        RoundedRectangle(cornerRadius: 10)
                                                            .stroke(Color(red: 0.851, green: 0.467, blue: 0.024).opacity(0.5), lineWidth: 1)
                                                    )
                                                    .font(.system(size: 13))
                                            }
                                            .padding(12)
                                            .background(Color(red: 1.0, green: 0.984, blue: 0.922))
                                            .cornerRadius(12)
                                            .overlay(
                                                RoundedRectangle(cornerRadius: 12)
                                                    .stroke(Color(red: 0.851, green: 0.467, blue: 0.024).opacity(0.3), lineWidth: 1)
                                            )
                                        } else {
                                            Text("We extract the reel's title and caption to check what is claimed.")
                                                .font(.system(size: 12))
                                                .foregroundColor(.textMuted)
                                                .lineSpacing(2)
                                        }
                                    }
                                default:
                                    TextEditor(text: $claimText)
                                        .frame(height: 100)
                                        .padding(10)
                                        .background(Color.bgSubtle)
                                        .cornerRadius(10)
                                        .overlay(
                                            RoundedRectangle(cornerRadius: 10)
                                                .stroke(Color.accentIndigo.opacity(0.4), lineWidth: 1)
                                        )
                                        .font(.system(size: 14))
                                }
                            }

                            // Quick examples
                            VStack(alignment: .leading, spacing: 8) {
                                Text("TRY A QUICK EXAMPLE")
                                    .font(.system(size: 10, weight: .bold))
                                    .foregroundColor(.textMuted)
                                    .tracking(0.6)
                                ScrollView(.horizontal, showsIndicators: false) {
                                    HStack(spacing: 8) {
                                        ExampleChip(label: "Health Myth", isDebunked: true) {
                                            activeTab = 0
                                            claimText = "WHO announces coffee cures Covid-19 mutations"
                                        }
                                        ExampleChip(label: "TikTok Reel", isDebunked: true) {
                                            activeTab = 2
                                            videoUrlText = "TikTok Reel spoken claim: Baking soda dissolves kidney stones in 24 hours"
                                        }
                                        ExampleChip(label: "YT Short", isDebunked: false) {
                                            activeTab = 2
                                            videoUrlText = "YouTube Short spoken claim: Overnight smartphone charging causes battery explosion"
                                        }
                                        ExampleChip(label: "Space", isDebunked: false) {
                                            activeTab = 0
                                            claimText = "James Webb Space Telescope detects atmospheric water vapor on exoplanet K2-18b"
                                        }
                                    }
                                }
                            }

                            if let err = errorMessage {
                                HStack(spacing: 6) {
                                    Circle()
                                        .fill(Color.colorPending)
                                        .frame(width: 7, height: 7)
                                    Text(err)
                                        .font(.system(size: 12))
                                        .foregroundColor(.colorPending)
                                }
                            }

                            // CTA button
                            Button(action: runVerification) {
                                ZStack {
                                    if isLoading {
                                        ProgressView()
                                            .progressViewStyle(CircularProgressViewStyle(tint: .white))
                                    } else if cooldownSeconds > 0 {
                                        Text("Cooldown (\(cooldownSeconds)s)...")
                                            .font(.system(size: 15, weight: .bold))
                                            .foregroundColor(.white)
                                    } else {
                                        Text("Check This Claim")
                                            .font(.system(size: 15, weight: .bold))
                                            .foregroundColor(.white)
                                    }
                                }
                                .frame(maxWidth: .infinity)
                                .frame(height: 52)
                                .background(
                                    cooldownSeconds > 0 || isLoading
                                    ? AnyShapeStyle(Color.gray)
                                    : AnyShapeStyle(
                                        LinearGradient(
                                            colors: [.accentIndigo, .accentViolet],
                                            startPoint: .leading,
                                            endPoint: .trailing
                                        )
                                    )
                                )
                                .cornerRadius(13)
                            }
                            .disabled(isLoading || cooldownSeconds > 0 || activeInput.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                        }
                        .padding(16)
                        .background(Color.bgCard)
                        .cornerRadius(16)
                        .shadow(color: Color.black.opacity(0.04), radius: 6, x: 0, y: 2)

                        // Results card
                        if let res = result {
                            ResultCard(res: res)
                        }

                        Spacer(minLength: 30)
                    }
                    .padding(.horizontal, 20)
                    .padding(.top, 16)
                }
            }
            .navigationTitle("Trace")
            .navigationBarTitleDisplayMode(.inline)
        }
    }

    private func runVerification() {
        let input = activeInput.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !input.isEmpty, !isLoading, cooldownSeconds == 0 else { return }
        isLoading = true
        errorMessage = nil
        needsManualClaim = false
        Task {
            do {
                // If video tab and no override, try metadata first
                let override = videoDialogueOverride.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty ? nil : videoDialogueOverride.trimmingCharacters(in: .whitespacesAndNewlines)
                if activeTab == 2, override == nil {
                    let meta = await GonkaService.shared.fetchVideoMetadata(input)
                    if meta == nil {
                        await MainActor.run {
                            self.needsManualClaim = true
                            self.isLoading = false
                        }
                        return
                    }
                }
                let res = try await GonkaService.shared.verifyClaim(claim: input, apiKey: nil, videoDialogueOverride: override)
                await MainActor.run {
                    self.result = res
                    self.isLoading = false
                    self.startCooldown()
                }
            } catch {
                await MainActor.run {
                    self.isLoading = false
                    self.errorMessage = "Notice: Check complete or brief rate limit active. Please retry in a moment."
                    self.startCooldown()
                }
            }
        }
    }

    private func startCooldown() {
        cooldownSeconds = 5
        Timer.scheduledTimer(withTimeInterval: 1.0, repeats: true) { timer in
            if cooldownSeconds > 0 {
                cooldownSeconds -= 1
            } else {
                timer.invalidate()
            }
        }
    }
}

// MARK: - Segmented Tab Row
struct SegmentedTabRow: View {
    let tabs: [String]
    @Binding var selectedIndex: Int

    var body: some View {
        HStack(spacing: 0) {
            ForEach(Array(tabs.enumerated()), id: \.offset) { index, label in
                Button(action: { selectedIndex = index }) {
                    Text(label)
                        .font(.system(size: 12, weight: selectedIndex == index ? .bold : .medium))
                        .foregroundColor(selectedIndex == index ? .white : .textMuted)
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 9)
                        .background(selectedIndex == index ? Color.accentIndigo : Color.clear)
                        .cornerRadius(8)
                }
            }
        }
        .background(Color.bgSubtle)
        .cornerRadius(10)
    }
}

// MARK: - Styled Text Field
struct StyledTextField: View {
    @Binding var text: String
    let placeholder: String

    var body: some View {
        ZStack(alignment: .leading) {
            if text.isEmpty {
                Text(placeholder)
                    .font(.system(size: 14))
                    .foregroundColor(.textMuted)
                    .padding(10)
            }
            TextField("", text: $text)
                .font(.system(size: 14))
                .padding(10)
        }
        .background(Color.bgSubtle)
        .cornerRadius(10)
        .overlay(
            RoundedRectangle(cornerRadius: 10)
                .stroke(Color.accentIndigo.opacity(0.4), lineWidth: 1)
        )
    }
}

// MARK: - Platform Chip
struct PlatformChip: View {
    let label: String

    var body: some View {
        Text(label)
            .font(.system(size: 12, weight: .medium))
            .foregroundColor(.textPrimary)
            .padding(.horizontal, 10)
            .padding(.vertical, 6)
            .background(Color.bgSubtle)
            .cornerRadius(8)
            .overlay(
                RoundedRectangle(cornerRadius: 8)
                    .stroke(Color.borderLight, lineWidth: 1)
            )
    }
}

// MARK: - Example Chip
struct ExampleChip: View {
    let label: String
    let isDebunked: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 6) {
                Circle()
                    .fill(isDebunked ? Color.colorDebunked : Color.colorVerified)
                    .frame(width: 7, height: 7)
                Text(label)
                    .font(.system(size: 12, weight: .medium))
                    .foregroundColor(.textPrimary)
            }
            .padding(.horizontal, 12)
            .padding(.vertical, 7)
            .background(Color.bgCard)
            .cornerRadius(20)
            .overlay(
                RoundedRectangle(cornerRadius: 20)
                    .stroke(Color.borderLight, lineWidth: 1)
            )
        }
    }
}

// MARK: - Results Card
struct ResultCard: View {
    let res: VerificationResult

    private var scoreColor: Color {
        if res.truthScore >= 80 { return .colorVerified }
        if res.truthScore >= 45 { return .colorPending }
        return .colorDebunked
    }

    private var scoreBg: Color {
        if res.truthScore >= 80 { return .colorGreenBg }
        if res.truthScore >= 45 { return Color(red: 1.0, green: 0.98, blue: 0.92) }
        return .colorRedBg
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {

            // Gauge + verdict row
            HStack(alignment: .center, spacing: 16) {
                // Circular gauge
                ZStack {
                    Circle()
                        .stroke(Color.borderLight, lineWidth: 7)
                        .frame(width: 84, height: 84)
                    Circle()
                        .trim(from: 0.0, to: CGFloat(res.truthScore) / 100.0)
                        .stroke(scoreColor, style: StrokeStyle(lineWidth: 7, lineCap: .round))
                        .frame(width: 84, height: 84)
                        .rotationEffect(.degrees(-90))
                    VStack(spacing: 0) {
                        Text("\(res.truthScore)%")
                            .font(.system(size: 19, weight: .black))
                            .foregroundColor(scoreColor)
                        Text("TRUTH")
                            .font(.system(size: 8, weight: .bold))
                            .foregroundColor(.textMuted)
                    }
                }

                VStack(alignment: .leading, spacing: 6) {
                    Text(res.verdictLabel)
                        .font(.system(size: 12, weight: .black))
                        .foregroundColor(scoreColor)
                        .padding(.horizontal, 10)
                        .padding(.vertical, 4)
                        .background(scoreBg)
                        .cornerRadius(20)

                    Text(res.headline)
                        .font(.system(size: 14, weight: .bold))
                        .foregroundColor(.textPrimary)
                        .lineSpacing(2)
                        .lineLimit(4)
                }
            }

            // Video spoken audio card
            if res.isVideo == true {
                VStack(alignment: .leading, spacing: 4) {
                    Text("\(res.videoPlatform ?? "Short Video"): Spoken Audio")
                        .font(.system(size: 11, weight: .bold))
                        .foregroundColor(.accentIndigo)
                    Text("\"\(res.spokenTranscript ?? res.claim)\"")
                        .font(.system(size: 13))
                        .foregroundColor(.textPrimary)
                        .lineSpacing(2)
                }
                .padding(12)
                .background(Color.colorIndigoBg)
                .cornerRadius(10)
                .overlay(
                    RoundedRectangle(cornerRadius: 10)
                        .stroke(Color.accentIndigo.opacity(0.25), lineWidth: 1)
                )
            }

            // Summary
            Text(res.summary)
                .font(.system(size: 13))
                .foregroundColor(.textSecondary)
                .lineSpacing(3)
        }
        .padding(20)
        .background(Color.bgCard)
        .cornerRadius(16)
        .shadow(color: Color.black.opacity(0.04), radius: 6, x: 0, y: 2)
    }
}
