# Midi

Midi is a CLI tool for midi devices. I have only tested it on linux at the moment. Midi is created with [midir](https://github.com/Boddlnagg/midir).

## Commands

List all midi devices on the machine:

```
❯ midi list
inputs:
	Midi Through:Midi Through Port-0 14:0
	CH345:CH345 MIDI 1 20:0
outputs:
	Midi Through:Midi Through Port-0 14:0
	CH345:CH345 MIDI 1 20:0
	VMPK Input:in 128:0
```

Send a test message to an input, will play a little song

```
❯ midi test "Midi Through:Midi Through Port-0 14:0"
```

Read the data from a midi output, will ignore clock and keep alive data.

```
❯ midi read "CH345:CH345 MIDI 1 20:0"
1352655: [144, 87, 80] (len = 3)
1363758: [144, 88, 80] (len = 3)
```

Relay all the data from one output to another input

```
❯ midi relay "CH345:CH345 MIDI 1 20:0" "Midi Through:Midi Through Port-0 14:0"
```

