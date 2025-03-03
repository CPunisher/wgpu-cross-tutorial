#ifndef libswift_binding_h
#define libswift_binding_h

void *init_wgpu(void* metal_layer, int width, int height);
void render(const void* wrapper);

#endif
