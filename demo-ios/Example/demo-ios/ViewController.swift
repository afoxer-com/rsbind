//
//  ViewController.swift
//  demo-swift
//
//  Created by sidney.wang on 2018/6/29.
//  Copyright © 2018年 sidney.wang. All rights reserved.
//

import UIKit
import rustlib

class ViewController: UIViewController {
    
    override func viewDidLoad() {
        super.viewDidLoad()
        let loginService = RustLib.newServices().getLoginService();
        let future = loginService.login(user_name: "sidney.wang", pwd: "88888888")
        let result = future.get();
        print("login result = \(result)")
        
        class Listener : UploadProgress {
            func onProgress(id: Int64, process: Int64, total: Int64) {
                print("Progress is \(process)/\(total)")
            }
        }
        let uploadService = RustLib.newServices().getUploadService();
        uploadService.upload(path: "to/your/path", listener: Listener())
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}
