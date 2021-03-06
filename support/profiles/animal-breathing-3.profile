id = '5cd23fa6-e965-45cb-a0cd-e87d28713095'
name = 'Animal: Breathing (3)'
description = 'Simulate a lifeform with organic movements'
active_scripts = [
	'swirl-perlin.lua',
	'animal.lua',
    'afterhue.lua',
    'macros.lua',
#   'stats.lua',
]

[[config."Perlin Swirl"]]
type = 'float'
name = 'color_divisor'
value = 2.0
default = 2.0

[[config."Perlin Swirl"]]
type = 'float'
name = 'color_offset'
value = -110.0
default = -110.0

[[config."Perlin Swirl"]]
type = 'float'
name = 'time_scale'
value = 250.0
default = 250.0

[[config."Perlin Swirl"]]
type = 'float'
name = 'coord_scale'
value = 15.0
default = 15.0

[[config."Perlin Swirl"]]
type = 'float'
name = 'opacity'
value = 0.1
default = 0.1

[[config.Animal]]
type = 'string'
name = 'name'
value = 'Breathing Blob'
default = 'Breathing Blob'

[[config.Animal]]
type = 'float'
name = 'speed'
value = 25.0
default = 25.0

[[config.Animal]]
type = 'float'
name = 'len_min'
value = 1.0
default = 1.0

[[config.Animal]]
type = 'float'
name = 'len_max'
value = 1.5
default = 1.5

[[config.Animal]]
type = 'color'
name = 'color_1'
value = 0xffffff00
default = 0xffffff00

[[config.Animal]]
type = 'float'
name = 'gradient_stop_1'
value = 0.03
default = 0.03

[[config.Animal]]
type = 'color'
name = 'color_2'
value = 0xff00ffff
default = 0xff00ffff

[[config.Animal]]
type = 'float'
name = 'gradient_stop_2'
value = 0.15
default = 0.15

[[config.Animal]]
type = 'color'
name = 'color_3'
value = 0xffff0a0a
default = 0xffff0a0a

[[config.Animal]]
type = 'float'
name = 'gradient_stop_3'
value = 0.5
default = 0.5

[[config.Animal]]
type = 'float'
name = 'coefficient_1'
value = 19.0
default = 19.0

[[config.Animal]]
type = 'float'
name = 'coefficient_2'
value = 7.0
default = 7.0

[[config.Animal]]
type = 'float'
name = 'coefficient_3'
value = 0.0
default = 0.0

[[config.Animal]]
type = 'float'
name = 'coefficient_4'
value = 1.0
default = 1.0

[[config.Animal]]
type = 'float'
name = 'coefficient_5'
value = 0.0
default = 0.0

[[config.Animal]]
type = 'float'
name = 'opacity'
value = 0.85
default = 0.85
