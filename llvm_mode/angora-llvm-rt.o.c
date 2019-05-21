#include <unistd.h>
#include <sys/shm.h>
#include "./config.h"

static u8 __angora_area_initial[MAP_SIZE];
u8 *__angora_area_ptr = __angora_area_initial;

static void map_shm(void) {
  char *id_str = getenv(SHM_ENV_VAR);
  if (id_str) {
    u32 shm_id = atoi(id_str);
    __angora_area_ptr = (u8 *)shmat(shm_id, NULL, 0);
    if (__angora_area_ptr == (void *)-1)
      _exit(1);
  }
}

extern void __angora_start_forkcli();

__attribute__((constructor(0))) void __angora_auto_init(void) {
  static u8 init_done;
  if (!init_done) {
    map_shm();
    __angora_start_forkcli();
    init_done = 1;
  }
}