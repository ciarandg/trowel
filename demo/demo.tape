Output demo.gif

Set Shell "bash"
Set FontSize 32
Set Width 1920
Set Height 1080
Set Theme "GruvboxDark"

Type "./result/bin/trowel-rs -p ./demo/plan.json" Sleep 2s  Enter

Sleep 2s

# Look at first resource in tree
Type "j" Sleep 500ms
Type "l" Sleep 5s
Type "h" Sleep 500ms

# Look at second resource in tree
Type "j" Sleep 500ms
Type "l" Sleep 10s
Type "h" Sleep 500ms

# Look at bottom resource in tree
Type "G" Sleep 500ms
Type "l" Sleep 10s
Type "h" Sleep 500ms

Sleep 5s
