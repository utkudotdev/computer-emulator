start:
    ssj
    ssf
    ldi x 0x8
    ldi y 0x4
    brn write_char.addr
    ldi x 0x9
    ldi y 0x6
    brn write_char.addr
    rsj
end:
    brn end.addr
write_char:
    mov z x
    out 0
    mov z y
    out 1
    sep 0    ; in the simulation delay doesn't really matter, but in real life we would want something here
    rsp 0
    ret