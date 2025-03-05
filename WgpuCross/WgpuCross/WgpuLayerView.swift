import SwiftUI
import MetalKit

struct WgpuLayerView: UIViewRepresentable {
    
    typealias UIViewType = MTKView
    
    func makeCoordinator() -> Coordinator {
        let coordinator = Coordinator()
        return coordinator
    }
    
    func makeUIView(context: Context) -> MTKView {
        let view = MTKView()
        view.delegate = context.coordinator
        view.device = MTLCreateSystemDefaultDevice()
        view.preferredFramesPerSecond = 60
        view.enableSetNeedsDisplay = false
        return view
    }
    
    func updateUIView(_ uiView: MTKView, context: Context) {
    }
    
    class Coordinator: NSObject, MTKViewDelegate {
        var wrapper: UnsafeMutableRawPointer?
        
        func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {}
        
        func draw(in view: MTKView) {
            if wrapper == nil {
                let metalLayer = Unmanaged.passUnretained(view.layer).toOpaque()
                wrapper = init_wgpu(metalLayer, Int32(view.frame.width), Int32(view.frame.height))
            }
            if let wrapper = wrapper {
                render(wrapper)
            }
        }
    }
}
