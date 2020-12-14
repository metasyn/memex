" output of $(pkg-config --cflags --libs MagickWand)
 let g:ale_c_cc_options="-fopenmp -DMAGICKCORE_HDRI_ENABLE=1 -DMAGICKCORE_QUANTUM_DEPTH=16 -fopenmp -DMAGICKCORE_HDRI_ENABLE=1 -DMAGICKCORE_QUANTUM_DEPTH=16 -I/usr/local/include/ImageMagick-7 -L/usr/local/lib -lMagickWand-7.Q16HDRI -lMagickCore-7.Q16HDRI"
