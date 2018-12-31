#!/usr/bin/env python3
# coding=utf-8
# python >= 3.5
"""
python3 lava_validation.py path-to-output-dir path-to-validated_bugs-file path-to-program [args..]
e.g.
python3 lava_validation.py ./output/ ./path-to-lava-M/who/validated_bugs ./who 
python3 lava_validation.py ./output/ ./path-to-lava-M/md5sum/validated_bugs ./md5sum -c

"""
import sys                                                                             
import os     
import subprocess        
import time                 
import shutil

def append_file(pstr, path):                    
    f = open(path, 'a')                           
    f.write("%s\n" % pstr)
    f.close()
    return               
                                     
def locate_crashes(crash_dirs, prom_bin, flags, save_dir):
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
                r = ""  
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
                            shutil.copy(cur_file, save_dir + "bug-" + str(cur_id))
                            bugs_id[cur_id] = 1
                        else:
                            bugs_id[cur_id] += 1       
                if has_crash_id == False and is_crash_dir:
                    print(cur_file + ": does not output bug id")
                    print(out)                 
    return bugs_id

if __name__ == "__main__":
    flags = []                                              
    fuzzer = ""                                                               
    prom = ""                                               
    output_dir = ""
    val_file = ""
    if len(sys.argv) > 3:
        output_dir = sys.argv[1]
        val_file = sys.argv[2]
        prom = sys.argv[3]
    else:
        print("The command format is : dir(e.g. output) validated_file(lava provide) prom(e.g. base64) {flags(-d)}")
        exit()
    if len(sys.argv) > 4:
        flags = sys.argv[4:]

    print("Target progrom is : ", prom, flags)

    val_ids = []
    extra_ids = []
    with open(val_file, 'r') as f:
        d = f.read()
        val_ids = list(map(int, d.split()))
        sorted(val_ids)

    unique_dir = output_dir + "/bugs/"
    if not os.path.isdir(unique_dir):
        os.mkdir(unique_dir)
    crash_dirs = [output_dir + "/crashes/", output_dir + "/queue/"]
    print(crash_dirs)
    log_file = output_dir + "/bug_log.txt"
    t0 = int(time.time())
    while True:
        print("-" * 80)
        t = int(time.time()) - t0
        print("time:", t);
        bugs_id = locate_crashes(crash_dirs, prom, flags, unique_dir)
        id_lists = list(bugs_id.keys())
        id_lists.sort()
        for i in id_lists:
            if i not in val_ids and i not in extra_ids:
                extra_ids.append(i)
        print("found ids:", id_lists)
        print("# of found ids: ", len(bugs_id))
        print("extra ids: ", extra_ids)
        print("fail to solve:", list(set(val_ids) - set(id_lists)))
        append_file(str(t) + "," + str(len(bugs_id)), log_file)
        time.sleep(30)
