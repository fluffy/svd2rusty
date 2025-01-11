
.phony: all


all:
	cargo run  STM32F0x2.svd  | rustfmt  > svd_stm32f0x2.rs
	cargo run STM32F405.svd| rustfmt  >  svd_stm32f405.rs


