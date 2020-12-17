//
//  RGBController.swift
//  RGBController
//
//  Created by Maxim Orlovsky on 12/16/20.
//

import Foundation
import rgblib

open class RGBController {
    private var client: COpaqueStruct
    let network: String
    let dataDir: String
    
    public init?(network: String = "testnet") {
        self.network = network ?? "testnet"
        self.dataDir = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!.path

        let client = rgb_node_run(self.network, self.dataDir)
        
        guard client.result.rawValue == 0 else {
            return nil
        }
        
        self.client = client.inner
    }
}
