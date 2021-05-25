# integrated circuits

i'm learning how to make my own [[diy-synthesizer]]. also see my notes on [[eletrical-engineering]].

# CMOS 4000 Series

* there are a lot of CMOS 4000 series ICs to do various things
* check [wikipedia](https://en.wikipedia.org/wiki/List_of_4000-series_integrated_circuits) for datasheets and descriptions

## CD4017BE

[datasheet](https://pdf1.alldatasheet.com/datasheet-pdf/view/26857/TI/CD4017BE.html)

* CMOS Counter/Divider
* also called a 5-stage johnson counter, or a decade counter
* 10 decoded ouputs
* the count is advanced one count at each positive clock signal transition, as long as the clock inhibit signal is low. otherwise, if the inhibit signal is high, the clock will not advance.
* sending a high signal to reset will clear the counter to the zero position

<img src="resources/img/cd4017be.png" width="300" />

## CD4040BE

[datasheet](https://pdf1.alldatasheet.com/datasheet-pdf/view/26879/TI/CD4040BE.html)

* CMOS Ripple-Carry Binary Counter/Divider
* takes input pulses and divides them into smaller / faster pulses
* each Q value = division by 2^Q

<img src="resources/img/cd4040be.png" width="300" />

## CD4046BE

[datasheet](https://www.alldatasheet.com/datasheet-pdf/pdf/26875/TI/CD4046BE.html)

* CMOS Micropower Phase-Locked Loop (PLL)
* has a voltage controlled oscillator (VCO) in it
* contains two phase comparators
* to use as an oscillator we need
    * one external capacitor (C1)
    * one/two external resistors (R1, R2)
    * C1, R1 determine frequency range
    * R2 determines frequency offset
* also features an on/off switch of sorts

<img src="resources/img/cd4046be.png" width="300" />

## C4051BE

[datasheet](https://pdf1.alldatasheet.com/datasheet-pdf/view/26882/TI/CD4051BE.html)

* CMOS Analog Multiplexers/Demultiplexers with Logic Level Conversion
* digitally controlled analog switches
* converts 3 bits of data (e.g. three square waves)
* A, B, C take digital signals (or square waves lol)
* these values are translated into one of the available channels out
* these can be reversed when used as demultiplexers
* `INH` is inhibit

<img src="resources/img/cd4051be.png" width="300" />

## CD4066BE

[datasheet](https://pdf1.alldatasheet.com/datasheet-pdf/view/26884/TI/CD4066BE.html)

* CMOS Quad Bilateral Switch
* intended for transmission or multiplexing of analog or digital signals
* pin for pin compatible with the CD4016B

<img src="resources/img/cd4066be.png" width="300" />

## CD4093BE

[datasheet](https://pdf1.alldatasheet.com/datasheet-pdf/view/834684/TI1/CD4093BE.html)

* CMOS Quad 2-Input NAND Schmitt Triggers
* has four distinct NAND gates in it
* can use this for making oscillators

<img src="resources/img/cd4093be.png" width="300" />

# Others

## LM386

[datasheet](http://www.taydaelectronics.com/datasheets/A-206.pdf)

* low voltage audio power amplifier
* can be used as an operational amplifier
* it also lets you specify the gain you want to add

<img src="resources/img/lm386.png" width="300" />

## L293DNE

[datasheet](https://pdf1.alldatasheet.com/datasheet-pdf/view/26889/TI/L293DNE.html)

* Quadruple Half-H Drivers

## LTV4N35

[datasheet](https://pdf1.alldatasheet.com/datasheet-pdf/view/325165/LITEON/LTV-4N35.html)

* General Purpose Photocoupler

<img src="resources/img/ltv4n35.png" width="300" />
