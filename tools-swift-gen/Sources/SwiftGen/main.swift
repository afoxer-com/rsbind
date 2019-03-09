import SwiftGenCore

if let tool = try? SwiftGen() {
    do {
        try tool.run()
    } catch {
        print("error when swift binder running: \(error)")
    }
}

