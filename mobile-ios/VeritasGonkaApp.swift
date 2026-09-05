import SwiftUI

@main
struct VeritasGonkaApp: App {
    var body: some Scene {
        WindowGroup {
            FactCheckView()
                .preferredColorScheme(.light) // Clean editorial light theme
        }
    }
}
