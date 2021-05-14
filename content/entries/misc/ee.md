# electrical engineering notes

i have slowly been learning how to create my own synthesizer and modules from scratch. however, i never studied electrical engineering, and generally am a complete novice. below are random notes i've been taking and resources i'm using.

# resources

* Moritz Klein - Building DIY VCO, VCF, etc. [Youtube](https://www.youtube.com/channel/UCzfW6SlNEyxmAPtdr3n-_Og) - [Patreon](https://www.patreon.com/moritzklein/posts)
* Look Mum No Computer - [Building your own modular synth](https://www.lookmumnocomputer.com/modular)
* Niklas Ronnberg - [Synth DIY Schematics](http://familjenronnberg.se/~niklas/diy.php)
* Natahn "Synthnerd" - [Synth DIY Schematics](https://synthnerd.wordpress.com/synth-diy/)


# laws

## ohms law

I = current (amperes)
V = voltage (volts)
R = resistance (ohms)

```
I = V / R
V = I * R
R = V / I
```

## faraday's law

predicts how a magnetic field will interact with an electric circuit to produce
a electromagnetic field, which is, electromagnetic induction

> The electromotive force around a closed path is equal to the negative of the
> time rate of change of the magnetic flux enclosed by the path.

# units

* Voltage (volts): electric potential difference
* Ampere (amps) = electrical current
* Ohm (Ω) = electrical resistance
* Farad = capacitance of electrical potential
* K = shorthand for 1000 Ohms often

# components

## diode

one side has no resistance, other is infinite (ideally).
they ensure current flows in one direction

cathode - short leg: negative end
anode - long leg: positive end

light emitting diode (LED) - converts electricity into light

## resistor

resistors simply resist current! then converts current into heat.

resistors are made out of insulators and conductors.
more insulators is equal to more resistance.

resistors have stripes on them. read about the [electronic color code](https://en.wikipedia.org/wiki/Electronic_color_code).

resistors also can serve as heating elements.
if you think about a toaster, or a hair dryer, they function by means of the same mechanism as resistors.

**water analogy**: smaller pipe = bigger resistor

## capacitors

stores electric charge in an electric field. it is :

electrolitic capacitors are bigger - they are polarized.
cermaic capacitors don't store very much charge - not polarized.

**water analogy**: a bucket that empties when full sorta?

## potentiometer

a three terminal resistor.

there is dial against a graphite disk
the more graphite that it has to go through, the more resistance.
the pivot point is connected to the middle.
side pins are the opposite sides.

the terms are called:
* nose of the potentiometer (middle)
* cheek of the potentiometer (ends)

## integrated circuit

a specific set of capacitors, inductors, transistors, resistors in a circuit.

these look like small "chips".  "living bug" position refers to it being with
its legs down.  and the flat part is at the top.

the pins on an IC have numbers like so:

```
10 9 8 7 6
 | | | | |
|----------|
|\         |
| |        |
|/         |
|----------|
 | | | | |
 1 2 3 4 5
```

where the left side has a marker like a half circle

## voltage divider

* passive linear circuit that reduces voltage from its input to its output
* one way to create a voltage divider is to use a potentiometer - a resistive divider
* If you set up two resistors - VoltageOut = Resistance2 / (Resistance1 + Resistance2) * VoltageIn


## op-amp

* op-amp is short for operational amplifier
* it can also be used as a buffer
* if you connect the negative out back to the post gain out it can be negative feedback
* using this buffer, we can add resistance to the feedback loop to control the total voltage added

# specific integrated circuits

## LM386

* low voltage audio power amplifier
* [datasheet](http://www.taydaelectronics.com/datasheets/A-206.pdf)

<img src="resources/img/dithered_lm386.png" />


## CD4093BE

* CMOS Quad 2-Input NAND Schmitt Triggers
* [datasheet](https://www.ti.com/lit/ds/symlink/cd4093b.pdf)

<img src="resources/img/dithered_cd4093be.png" />


## CMOS 4000 Series

* there are a lot of CMOS 4000 series ICs to do various things
* check [wikipedia](https://en.wikipedia.org/wiki/List_of_4000-series_integrated_circuits) for datasheets and descriptions


# schematics

within electrical engineering, we need to read schematics. they're designs that
specify the components of a electrical circuit. reading schematics mostly
involves learning a number of small icons used as a visual language to
represent components. see this useful
[sparkfun](https://learn.sparkfun.com/tutorials/how-to-read-a-schematic/all)
guide on reading schematics.

# bom

BOM usually stands for bill of materials - a list of all the components or eletronics or pieces used in a circuit or build.
