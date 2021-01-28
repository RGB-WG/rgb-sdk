//
//  RGBController.swift
//  RGBController
//
//  Created by Maxim Orlovsky on 12/16/20.
//

import Foundation
import rgblib

public struct RGBError: Error {
    let message: String
    
    //Temporary solution for getting a string from a pointer without knowing the byte count
    private static func extractString(_ ptr: UnsafeRawPointer) -> String {
        var message = ""
        
        var byteCount: Int = 1
        while byteCount < 1000 {
            let data = Data(bytes: ptr, count: byteCount)
            guard let m = String(data: data, encoding: String.Encoding.utf8) else {
                //If we've gone too far and string is nil
                break
            }
            
            byteCount += 1
            message = m
        }
        
        return message
    }
    
    init!(_ res: CResult) {
        guard res.result.rawValue != 0 else {
            return nil
        }
            
        //TODO Real solution is this but we need the real byteCount returned as part of COpaqueStruct
//        let byteCount =
//        let data = Data(bytes: res.inner.ptr, count: byteCount)
//        self.message = String(data: data, encoding: String.Encoding.utf8) ?? "Unknown RGB Error"
        
        self.message = RGBError.extractString(res.inner.ptr)
    }
    

    init!(_ res: CResultString) {
        guard res.result.rawValue != 0 else {
            return nil
        }
        self.message = String(cString: res.inner)
    }
    
    init(_ msg: String) {
        self.message = msg
    }
}

extension RGBError: LocalizedError {
    public var errorDescription: String? {
        return self.message
    }
}

public enum Verbosity: UInt8 {
    case Error = 0
    case Warning = 1
    case Info = 2
    case Debug = 3
    case Trace = 4
}

open class RGB20Controller {
    private var client: COpaqueStruct
    let network: String
    let dataDir: String
    
    public init(network: String = "testnet", electrum: String = "pandora.network:60001", verbosity: Verbosity = .Info) throws {
        self.network = network
        self.dataDir = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!.path

        let client = rgb_node_run(self.dataDir, self.network, electrum, verbosity.rawValue)
        
        guard client.result.rawValue == 0 else {
            throw RGBError(client)
        }
        
        self.client = client.inner
    }
    
    open func createAsset(ticker: String, name: String, description: String? = nil, precision: UInt8 = 8, allocations: String = "[]") throws {
        // let allocations = String(data: try JSONEncoder().encode(allocations), encoding: .utf8)
        
        try withUnsafePointer(to: self.client) { client in
            let res = rgb_node_fungible_issue(
                client,
                network,
                ticker,
                name,
                description,
                precision,
                allocations,
                "[]", "null", "null")
            guard res.result.rawValue == 0 else {
                throw RGBError(res)
            }
        }
    }
    
    open func listAssets() throws -> String {
        try withUnsafePointer(to: self.client) { client in
            let res = rgb_node_fungible_list_assets(client)
            guard res.result.rawValue == 0 else {
                throw RGBError(res)
            }
            guard let jsonString = String(utf8String: res.inner) else {
                throw RGBError("Wrong node response (not JSON string)")
            }
            //try JSONSerialization.jsonObject(with: jsonString.data(using: .utf8), options: [])
            return jsonString
        }
    }
}
