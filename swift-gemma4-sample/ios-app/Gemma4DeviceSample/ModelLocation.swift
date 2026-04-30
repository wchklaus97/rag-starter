import Foundation
import Gemma4SampleCore

enum ModelLocation {
    static let folderName = "gemma-4-e2b-it-4bit"

    static func firstAvailableModelSource() throws -> Gemma4ModelSource {
        for url in candidateDirectories() where isModelDirectory(url) {
            return .localDirectory(url)
        }

        throw ModelLocationError.modelDirectoryMissing(candidates: candidateDirectories())
    }

    static func candidateDirectories() -> [URL] {
        var urls: [URL] = []

        if let documents = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first {
            urls.append(documents.appending(path: folderName, directoryHint: .isDirectory))
        }

        if let bundleResource = Bundle.main.resourceURL {
            urls.append(bundleResource.appending(path: folderName, directoryHint: .isDirectory))
        }

        return urls
    }

    private static func isModelDirectory(_ url: URL) -> Bool {
        let requiredFiles = [
            "config.json",
            "tokenizer.json",
            "model.safetensors",
        ]

        return requiredFiles.allSatisfy { fileName in
            FileManager.default.fileExists(atPath: url.appending(path: fileName).path)
        }
    }
}

enum ModelLocationError: LocalizedError {
    case modelDirectoryMissing(candidates: [URL])

    var errorDescription: String? {
        switch self {
        case .modelDirectoryMissing(let candidates):
            let paths = candidates.map(\.path).joined(separator: "\n")
            return """
            Model directory missing. Copy gemma-4-e2b-it-4bit into one of:
            \(paths)
            """
        }
    }
}
