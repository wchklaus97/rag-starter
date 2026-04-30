import XCTest
@testable import Gemma4SampleCore

final class Gemma4SampleTests: XCTestCase {
    func testConfigWithRemoteModel() {
        let config = Gemma4SampleConfig(environment: [:])
        XCTAssertTrue(config.usesRemoteModel)
        XCTAssertEqual(config.modelSource, .verifiedHuggingFace)
    }
    
    func testConfigWithLocalModel() {
        let path = "/path/to/local/model"
        let config = Gemma4SampleConfig(environment: ["GEMMA4_MODEL_DIR": path])
        XCTAssertFalse(config.usesRemoteModel)
        XCTAssertEqual(config.modelSource, .localDirectory(URL(fileURLWithPath: path)))
    }
    
    func testConfigWithEmptyLocalModel() {
        let config = Gemma4SampleConfig(environment: ["GEMMA4_MODEL_DIR": ""])
        // Empty string should be treated the same as missing — falls back to remote
        XCTAssertTrue(config.usesRemoteModel)
        XCTAssertEqual(config.modelSource, .verifiedHuggingFace)
    }
    
    func testEventMessages() {
        XCTAssertEqual(Gemma4RuntimeEvent.starting.message, "Starting Gemma4 sample.")
        XCTAssertEqual(Gemma4RuntimeEvent.loadingModel("test").message, "Loading model: test")
        XCTAssertEqual(Gemma4RuntimeEvent.promptEncoded(42).message, "Prompt encoded: 42 tokens.")
    }
}
