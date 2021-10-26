#!/usr/bin/env bash

declare -a COLOR_LIST=(
  'EMERALD+63+195+128+1' 
  'LIGHT_SEA_GREEN+27+163+156+1'
  'NIAGARA+42+187+155+1'
  'JAVA+35+203+167+1'
  'REBECCA_PURPLE+102+51+153+1'
  'ELECTRIC_INDIGO+140+20+252+1'
  )

for COLOR in ${COLOR_LIST[*]}; do
  NAME=$(echo $COLOR | cut -d '+' -f 1)
  RED=$(echo $COLOR | cut -d '+' -f 2)
  GREEN=$(echo $COLOR | cut -d '+' -f 3)
  BLUE=$(echo $COLOR | cut -d '+' -f 4)
  ALPHA=$(echo $COLOR | cut -d '+' -f 5)
  cat << EOF >> gen_structs.rs
  pub const ${NAME}: RGBA = RGBA(ffi::GdkRGBA {
      red: $(python -c "red=(${RED}/255); print('{:.3f}'.format(red))")f32,
      green: $(python -c "green=(${GREEN}/255); print('{:.3f}'.format(green))")f32,
      blue: $(python -c "blue=(${BLUE}/255); print('{:.3f}'.format(blue))")f32,
      alpha: $(python -c "alpha=${ALPHA}; print('{:.3f}'.format(alpha))")f32,
  });
EOF
done
