
#ifndef LOGGER_H
#define LOGGER_H

#include "cond_stmt.h"
#include "libdft_api.h"
#include <set>

#define TRACK_COND_OUTPUT_VAR "ANGORA_TRACK_OUTPUT"

#define BUF_LEN (2 << 16)
class LogBuf {
private:
  char *buffer;
  u32 cap;
  size_t len;

public:
  void push_bytes(char *bytes, std::size_t size) {
    if (size > 0 && bytes) {
      size_t next = len + size;
      if (next > BUF_LEN) {
        cap *= 2;
        buffer = (char *)realloc(buffer, cap);
      }
      memcpy(buffer + len, bytes, size);
      len = len + size;
    }
  };

  void write_file(FILE *out_f) {
    if (!out_f || len == 0)
      return;
    int nr = fwrite(buffer, len, 1, out_f);
    if (nr < 1) {
      fprintf(stderr, "fail to write file %d %lu\n", nr, len);
      exit(1);
    }
  };

  LogBuf() {
    cap = BUF_LEN;
    buffer = (char *)malloc(cap);
    len = 0;
  };
  ~LogBuf() { free(buffer); }
};
class Logger {
private:
  u32 num_cond;
  u32 num_tag;
  u32 num_mb;
  LogBuf cond_buf;
  LogBuf tag_buf;
  LogBuf mb_buf;
  std::map<u64, u32> order_map;
  std::set<lb_type> lb_set;

public:
  Logger(){};
  ~Logger(){};
  void save_buffers() {
    FILE *out_f = NULL;
    char *track_file = getenv(TRACK_COND_OUTPUT_VAR);
    if (track_file) {
      out_f = fopen(track_file, "w");
    } else {
      out_f = fopen("track.out", "w");
    }

    fwrite(&num_cond, 4, 1, out_f);
    fwrite(&num_tag, 4, 1, out_f);
    fwrite(&num_mb, 4, 1, out_f);

    cond_buf.write_file(out_f);
    tag_buf.write_file(out_f);
    mb_buf.write_file(out_f);

    if (out_f) {
      fclose(out_f);
      out_f = NULL;
    }
  }

  u32 get_order(u32 cid, u32 ctx) {
    u64 key = cid;
    key = (key << 32) | ctx;
    u32 ctr = 1;
    if (order_map.count(key) > 0) {
      ctr = order_map[key] + 1;
      order_map[key] = ctr;
    } else {
      order_map.insert(std::pair<u64, u32>(key, 1));
    }
    return ctr;
  }
  void save_tag(lb_type lb) {
    if (lb > 0 && lb_set.count(lb) == 0) {
      std::vector<tag_seg> t = tag_get(lb);
      u32 n = t.size();
      tag_buf.push_bytes((char *)&lb, 4);
      tag_buf.push_bytes((char *)&n, 4);
      tag_buf.push_bytes((char *)&t[0], sizeof(tag_seg) * n);
      num_tag += 1;
      lb_set.insert(lb);
    }
  };

  void save_mb(u32 size, char *arg1, char *arg2) {
    if (num_cond > 0) {
      u32 i = num_cond - 1;
      mb_buf.push_bytes((char *)&i, 4);
      mb_buf.push_bytes((char *)&size, 4);
      mb_buf.push_bytes(arg1, size);
      mb_buf.push_bytes(arg2, size);
    }
  };

  void save_cond(CondStmt &cond) {
    save_tag(cond.lb1);
    save_tag(cond.lb2);
    num_cond += 1;
    cond_buf.push_bytes((char *)&cond, sizeof(CondStmt));
  }
};

/*
// fn with magic bytes
void CircBuf::push_cond(CondStmt &cond, u8 *arg1, u8 *arg2) {
  std::lock_guard<std::mutex> lock(c_mutex);
  add_label(cond.lb1);
  add_label(cond.lb2);
  push_bytes((u8 *)&cond, sizeof(CondStmt));
  std::size_t size = cond.size;
  push_bytes(arg1, size);
  push_bytes(arg2, size);
}

void CircBuf::push_switch(CondStmt &cond, std::size_t num, u64 *args) {
  std::lock_guard<std::mutex> lock(c_mutex);
  add_label(cond.lb1);
  for (int i = 0; i < num; i++) {
    if (cond.condition == args[i])
      cond.condition = 3;
    else
      cond.condition = 1;
    cond.arg2 = args[i];
    // Switch -> If
    // distinguish them by "i".
    // FIXME:
    // cond.order = (cond.order & 0xFFFF) | (i << 16);
    cond.order = (i << 16);
    push_bytes((u8 *)&cond, sizeof(CondStmt));
  }
}

void CircBuf::save_labels() {
  // save an zero cond_stmt to indicate it should finish parse cond
  CondStmt cond;
  memset(&cond, 0, sizeof(CondStmt));
  push_bytes((u8 *)&cond, sizeof(CondStmt));

  // strucutre: {lable_id (u32), seg_len(u32), segs}
  for (u32 i = 1; i <= max_label; i++) {
    if (labels[i]) {
      push_bytes((u8 *)&i, 4);
      auto off = dfsan_get_label_lens(i);
      u32 size = off.size();
      push_bytes((u8 *)&size, 4);
      push_bytes((u8 *)&off[0], sizeof(tag_seg) * size);
    }
  }

  // finish with: {0, 0}, meaning label_id is 0, and size is 0, so we have to
  // say goodbye.
  u8 fin_bytes[8] = {0};
  push_bytes(fin_bytes, 8);
}
*/

#endif