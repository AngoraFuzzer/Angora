#!/usr/bin/env python3
# coding=utf-8

# python >= 3.5
import sys
import os
import subprocess
import time

def append_file(pstr, path):
    f = open(path, 'a')
    f.write("%s\n" % pstr)
    f.close()
    return

def locate_crashes(crash_dirs, prom_bin, flags):
    bugs_id = {}
    for cur_dir in crash_dirs:
        is_crash_dir = cur_dir.endswith("crashes/")
        for file in os.listdir(cur_dir):
            if (file != "README.txt"):
                cur_file = cur_dir + file
                cmd = [prom_bin]
                for flag in flags:
                    cmd.append(flag)
                cmd.append(cur_file)
                try:
                    r = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, timeout=4)
                except subprocess.TimeoutExpired:
                    print("time out")
                    continue

                out = r.stdout.split(b'\n')
                has_crash_id = False
                for line in out:
                    if line.startswith(b"Successfully triggered bug"):
                        dot = line.split(b',')[0]
                        cur_id = int(dot[27:])
                        has_crash_id = True
                        if cur_id not in bugs_id:
                            # print(cmd, cur_id)
                            bugs_id[cur_id] = 1
                        else:
                            bugs_id[cur_id] += 1
                if has_crash_id == False and is_crash_dir:
                    print(cur_file + ": has not crash id")
                    print(out)
    return bugs_id

if __name__ == "__main__":
    flags = []
    fuzzer = ""
    prom = ""
    output_dir = ""
    if len(sys.argv) > 2:
        output_dir = sys.argv[1]
        prom = sys.argv[2]
    else:
        print("The command format is : dir(e.g. output) prom(e.g. base64) {flags(-d)}")
        exit()
    if len(sys.argv) > 3:
        flags = sys.argv[3:]

    # prom_bin = "./" + prom + "/" + "origin" + "/lava-install/bin/" + prom
    prom_base = "./LAVA-M/" + prom + "/"
    prom_bin = prom_base + "log" + "/lava-install/bin/" + prom
    print("Target progrom is : ", prom_bin, flags)
    base_dir = "./tests/"

    if "/" in fuzzer:
        crash_base = fuzzer + "/"
    else:
        crash_base = base_dir + prom + "/" + output_dir + "/"

    val_ids = []
    extra_ids = []
    with open(prom_base + "validated_bugs", 'r') as f:
        d = f.read()
        val_ids =  list(map(int, d.split()))
        sorted(val_ids)
        #print("val id:", val_ids)

    sdirs = os.listdir(crash_base)
    if "queue" in sdirs:
        sdirs = {"",}
    crash_dirs = [crash_base + "crashes/", crash_base + "queue/"]
    print(crash_dirs)
    log_file = crash_base + "/bug_log.txt"
    t0 = int(time.time())
    while True:
        print("-" * 80)
        t = int(time.time()) - t0
        print("time:", t);
        bugs_id = locate_crashes(crash_dirs, prom_bin, flags)
        id_lists = list(bugs_id.keys())
        id_lists.sort()
        for i in id_lists:
            if i not in val_ids and i not in extra_ids:
                extra_ids.append(i)
        print("found ids:", id_lists)
        print("# of found ids: ", len(bugs_id))
        print("extra ids: ", extra_ids)
        append_file(str(t) + "," + str(len(bugs_id)), log_file)
        time.sleep(30)
