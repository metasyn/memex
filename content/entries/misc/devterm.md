# devterm

a while back clockworkpi released a small handheld computer called the [devterm](https://www.clockworkpi.com/devterm). i originally bought it for the inclusion with my eurorack synthesizer, hoping to potentially use it as a small and quick way to add [orca](https://100r.co/site/orca.html). if you haven't used orca before, check out my project [[learn-orca]] too. in particular, i was interested in a compact screen and keyboard combo, which fits the bill for the devterm. i also thought it might be an interesting computer to bring to certain situations, where i don't necessarily need the entirety of a full laptop/computer to accomplish some task. it is really small and light!

upon receiving it though, i realized i would need to customize it a little to
feel comfortable using it. below are some various notes on getting it setup and
making it useful for myself. there is also a [wiki](https://wiki.clockworkpi.com/index.php/Main_Page#DevTerm) with some useful information.

## hardware switch for gamepad buttons

one thing that took me a little bit to figure out was switching this hardware switch while assembling it to enable the gamepad buttons and d-pad style arrows to work as key signals (instead of whatever they were sending before). i ended up learning this via the forum - see [here](https://forum.clockworkpi.com/t/using-gamepad-arrows-and-buttons-in-command-line-apps/7059/2) for a nice picture. this allows the gamepad  buttons to send the following keys:

* a button - *j* key
* b button - *k* key
* x button - *u* key
* y button - *i* key

## xfce updating

* update xfce theme - its pretty ugly otherwise
* update xfce window manager keyboard combinations - they keyboard layout doesn't really allow for a ton of the defaults to make sense. i remapped how i can shift windows around and maximize, minimize, etc without having to use anything other than Ctrl / Super / Alt and the arrow keys.
* i decided to use [rofi](https://github.com/davatorium/rofi) as a way of launching applications easily, since using the trackball can be a little tedious sometimes
* i bound "Super + Space" to `rofi -show combi -combi-modes "window,run,drun" -modes combi` which is a really nice TUI style interface (though its a window) to launch and select various applications that are installed (including tons of xfce related settings etc)
