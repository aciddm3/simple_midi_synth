# aciddm project "Daleth"

Experimental additive synthesizer written in Rust

## Main information

Daleth is a additive synthesizer with *"common envelope"* modulation features.

### Common envelope

When we have a deal with a synth envelope like ADSR, DAHSDR and other, we are just adjusting two functions:

- one -- when note is not pressed; we are adjusting it with Release parameter.
- second -- when note is pressed; we are adjusting it with Delay, Attack, Hold, Sustain, Decay parameters.

But what if we want to expand the concept of envelope?
Let there are 2 math function $F$ and $N$, which can be defined with expression.

So? common envelope is a envelope? which value equals :

- $F(t)$ -- when note is not pressed,
- $N(t)$ -- when note is pressed,

there is $t$ is time since note on/off moment.

So, you can define your own function for each sine signal.
Yo can change pitch, phase and amplitude.

```json
[ <- list of sines
 { <- Sine 1
 "pitch_gate_on": "(oct (/ (exp (* arg -4.0)) 100.0) freq)",
 "pitch_gate_off": "0.0",
 "pitch_slew_limit" : 1.0,
 ^ octave shift
 "phase_gate_on": "0.0",
 "phase_gate_off": "0.0",
 "phase_slew_limit" : 1.0,
 ^ phase shift
 "amplitude_gate_on": "(sigmoid arg)",
 "amplitude_gate_off": "(exp (const-mul arg -4.0))",
 "amplitude_slew_limit" : 0.1
 ^ amplitude shift
 },
 { <- Sine 2
 "pitch_gate_on": "(* 3 freq)",
 ^ absolute frequency (Hz)
 "pitch_gate_off": "0.0",
 "pitch_slew_limit" : 1.0,
 "phase_gate_on": "0.0",
 "phase_gate_off": "0.0",
 "phase_slew_limit" : 1.0,
 "amplitude_gate_on": "(const-mul (sigmoid arg) 0.5)",
 "amplitude_gate_off": "(const-mul (exp (const-mul arg -4.0)) 0.5)",
 "amplitude_slew_limit" : 0.1
 }
]
```

There is slew limiter for each envelope. It makes envelope smoother.

Function defined by Sexpr, ypu can view list of them in file "s_expr_help_ru.xlsx" in Russian. Use Translator.

## Dependencies and crates

Used these crates:

- nih-plug (main engine)
- nih-plug egui (GUI)
- aciddm3_atom_func (function shell)
- parking_lot (RWLock)
- serde (serialize)
- rfd (Rust file dialog)
