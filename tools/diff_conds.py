#!/usr/bin/env python3
# coding=utf-8

"""
find failed constraints
python3 ~/angora/tools/diff_conds.py a.csv b.csv
"""

import sys

def get_conds(f):
    file = open(f, "r")
    buf = file.read()
    file.close()
    lines = buf.split("\n")
    conds = []
    for l in lines:
        ent = l.split(',')
        if ent[0] == "cmpid":
            continue
        if len(ent) == 1:
            continue
        if len(ent) > 3 and int(ent[3]) < 1000:
            continue
        print(ent[0], ent[1], ent[2])
        conds.append((int(ent[0]), int(ent[1]), int(ent[2])))

    return conds

if len(sys.argv) == 3:
    f1 = sys.argv[1]
    f2 = sys.argv[2]
else:
    print("wrong input file?")
    exit(1)

conds1 = set(get_conds(f1))
conds2 = set(get_conds(f2))

if conds1 == conds2:
    print("conds1 == conds2")
elif conds1 < conds2:
    print("cond1 < cond2")
    print("conds2 - conds1 is ", conds2 - conds1)
elif conds1 > conds2:
    print("conds1 > conds2")
    print("conds1 - conds2 is ", conds1 - conds2)
else:
    print("conds1 has ", conds1 - conds2)
    print("conds2 has ", conds2 - conds1)

conds3 = [",".join(map(str, x)) for x in list(conds1 | conds2)]
content = "\n".join(conds3)
file = open("global.csv", "w")
file.write(content)
file.close()
