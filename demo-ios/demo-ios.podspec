#
# Be sure to run `pod lib lint demo-ios.podspec' to ensure this is a
# valid spec before submitting.
#
# Any lines starting with a # are optional, but their use is encouraged
# To learn more about a Podspec see https://guides.cocoapods.org/syntax/podspec.html
#

Pod::Spec.new do |s|
  s.name             = 'demo-ios'
  s.version          = '0.1.0'
  s.summary          = 'A short description of demo-ios.'

# This description is used to generate tags and improve search results.
#   * Think: What does it do? Why did you write it? What is the focus?
#   * Try to keep it short, snappy and to the point.
#   * Write the description between the DESC delimiters below.
#   * Finally, don't worry about the indent, CocoaPods strips it!

  s.description      = <<-DESC
TODO: Add long description of the pod here.
                       DESC

  s.homepage         = 'https://github.com/sidneywang/demo-ios'
  # s.screenshots     = 'www.example.com/screenshots_1', 'www.example.com/screenshots_2'
  s.license          = { :type => 'MIT', :file => 'LICENSE' }
  s.author           = { 'sidneywang' => 'sidney.wang@foxmail.com' }
  s.source           = { :git => 'https://github.com/sidneywang/demo-ios.git', :tag => s.version.to_s }
  # s.social_media_url = 'https://twitter.com/<TWITTER_USERNAME>'

  s.ios.deployment_target = '9.0'

  s.source_files = 'demo-ios/Classes/**/*', 'rustlib/_gen/ios_artifact/rustlib/*.{swift,m}'

  s.subspec 'rustlib' do |ss|
    ss.vendored_libraries = 'rustlib/_gen/ios_artifact/rustlib/*.a'
    ss.source_files = 'rustlib/_gen/ios_artifact/rustlib/*.h'
    ss.public_header_files = 'rustlib/_gen/ios_artifact/rustlib/*.h'
    ss.xcconfig = {
          'HEADER_SEARCH_PATHS' => '"rustlib/_gen/ios_artifact/rustlib/"'
    }
#    s.pod_target_xcconfig = { 'SWIFT_OBJC_BRIDGING_HEADER' => 'rustlib/_gen/ios_artifact/rustlib/rustlib_bridge.h' }
  end
  
  # s.resource_bundles = {
  #   'demo-ios' => ['demo-ios/Assets/*.png']
  # }

  # s.public_header_files = 'Pod/Classes/**/*.h'
  # s.frameworks = 'UIKit', 'MapKit'
  # s.dependency 'AFNetworking', '~> 2.3'
end
