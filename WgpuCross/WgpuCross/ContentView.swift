import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            WgpuLayerView()
                .frame(width: 400, height: 400)
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
