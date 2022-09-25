## Custom Animations
msi-klc allows you to use custom animations.\
The `wave` and `breathe` animations have been created as an example for you to create your own animations.

### Documentation
`loop_forever` - If this line is anywhere in your animation file, your animation will loop forever.

`reset` - Makes all the LEDs on your keyboard white.\
`off` - Turns off all the LEDs on your keyboard.

`color:#<hex code>`/`color:<red>;<green>;<blue>` - Changes the color of your entire keyboard.\
`region:<left/middle/right>` - Used with `color` to indicate the region you want to change.\
`sleep:<milliseconds>` - Sleep for the specified amount of milliseconds.

