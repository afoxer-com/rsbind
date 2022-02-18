//
//  SynchronizedClosure.swift
//  DocsSDK
//
//  Created by zenghao on 2018/8/19.
//

import Foundation

let sema = DispatchSemaphore(value: 1)

func synchronized(_ closure: () -> ()) {
    sema.wait()
    closure()
    sema.signal()
}
