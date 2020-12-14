#include <stdio.h>
#include <stdlib.h>
#include <MagickWand/MagickWand.h>

#define ThrowWandException(wand) \
{ \
  char \
    *description; \
 \
  ExceptionType \
    severity; \
 \
  description=MagickGetException(wand,&severity); \
  (void) fprintf(stderr,"%s %s %lu %s\n",GetMagickModule(),description); \
  description=(char *) MagickRelinquishMemory(description); \
  exit(-1); \
}

void
check(MagickWand *magick_wand, MagickBooleanType status) {
  if (status == MagickFalse) ThrowWandException(magick_wand);
}


/*

#!/usr/bin/env bash

# This function basically does the following
convert $1 \
  -resize 400 \
  -posterize 24 \
  -ordered-dither o3x3 \
  -quality 70 \
  -strip $PREFIX_$1.png

 */
void
convertFile(char *output_path, char *prefix, char *input_file_path) {

  ///////////
  // INPUT //
  ///////////
  MagickWand* magick_wand = NewMagickWand();
  MagickBooleanType status = MagickReadImage(magick_wand, input_file_path);
  check(magick_wand, status);

  // Allocate a new char array for the new file path
  char *file_name = basename(input_file_path);
  char output_file[strlen(output_path) + 1 + strlen(prefix) + strlen(file_name) + 1];
  snprintf(output_file, sizeof output_file, "%s/%s%s", output_path, prefix, file_name);
  printf("-> new file: %s\n", output_file);

  ////////////
  // RESIZE //
  ////////////

  // max width
  size_t max_width = 400;

  // Figure out existing dimensions
  size_t original_width = MagickGetImageWidth(magick_wand);
  size_t original_height = MagickGetImageHeight(magick_wand);

  if (original_width > max_width)  {
    size_t new_height= original_height * (max_width / original_width);
    // Convert images
    MagickResizeImage(magick_wand, max_width, new_height, LanczosFilter);
  }

  ///////////////
  // POSTERIZE //
  ///////////////
	status = MagickPosterizeImage(magick_wand, 24, NoDitherMethod);
  check(magick_wand, status);

  ///////////////
  // DITHERING //
  ///////////////
	status = MagickOrderedDitherImage(magick_wand, "o3x3");
  check(magick_wand, status);

  /////////////
  // QUALITY //
  /////////////
	status = MagickSetImageCompressionQuality(magick_wand, 70);
  check(magick_wand, status);

  ///////////
  // STRIP //
  ///////////
	status = MagickStripImage(magick_wand);
  check(magick_wand, status);

  // Write file, check
  status = MagickWriteImages(magick_wand, output_file, MagickTrue);
  check(magick_wand, status);

  ////////////
  // FINISH //
  ////////////
  DestroyMagickWand(magick_wand);
}

int
convert(char *output_path, char *prefix, int file_count, char **input_files){
  // setup
  MagickWandGenesis();

  // loop over files
  for (int i = 0; i < file_count; i++) {
    printf("Downscaling: %s\n", input_files[i]);
    convertFile(output_path, prefix, input_files[i]);
  }

  // cleanup
  MagickWandTerminus();
  return 0;
}
