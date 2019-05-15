# rustylink

`rustylink` is a STM32 devices interface written purely in Rust. It can read and write into the device's SRAM section, read registers, dump the devices register status (r0, r1, etc...) and read and write the Flash memory.

## Device support

Right now, `rustylink` supports the following devices:
 * STM32 F0 line.
 * STM32 F7 line.
 * STM32 F3 line.

Partial support:
 * STM32 G0 line. Value line not supported.

WIP:
  * STM32 F4 line.
  * STM32 H7 line.
  
  ## LICENSE
  `rustylink` is currently licensed under the MIT license. Any work submitted to this repository will be licensed under the application's license.
