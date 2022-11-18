# TwoFrame
I made this project to visualize inputs per frame as they come in from a controller.
I needed this since most fighting games don't include a history of inputs on a per-frame basis.


Currently, the program only:
* shows inputs specific to **Guilty Gear Strive**. 
* responds to / is configured for a PS4 controller connected via USB

Though these can all be adjusted to allow for custom input representation and bindings.
Example of myself trying a difficult combo link:
```
S----------------------]H[,S----------------------]H[----P
S---------------------]H[S------------------------]H[---P
S---------------------]H[--S-----------------------]H[----P
S----------------------]H[,S----------------------]H[----P
S----------------------]H[-S-----------------------]H[---P
S-------------------------]H[S-----------------------]H[--P
S-------------------------]H[S------------------------]H[--P
S----------------------]H[,S----------------------]H[----P
S-----------------------]H[S----------------------]H[----P
```

## How To Use
1. Go to the releases page on the right. Look for the most up-to-date version, something
`twoframe v1.0.0`.
2. Under the **Assets** section, download `twoframe.exe`.
3. Place the program wherever you want.
4. To use your binds: create a text file called `twoframe_config.txt` in the same directory as the program, ie. beside it.
5. Add these contents to the file and nothing more, nothing less: 
```json
{
    "h_bind": 11,
    "k_bind": 13,
    "d_bind": 14,
    "p_bind": 15,
    "s_bind": 16
}
```
Now which numbers are which buttons? See here:
```
1 => up_left_button
2 => up_right_button
3 => down_left_button
4 => down_right_button
5 => left_button
6 => right_button
7 => up_button
8 => down_button

--- Important ones ---
9 => l1_button
10 => l2_button
11 => r1_button
12 => r2_button
13 => x_button
14 => o_button
15 => square_button
16 => triangle_button
----------------------

17 => share_button
18 => option_button
20 => left_stick_button
21 => right_stick_button
```
6. Run the program!

**NOTE**: Any time you press the **Share** button on the PS4 controller, the program will
print out your inputs stored and begin recording a new set of inputs.