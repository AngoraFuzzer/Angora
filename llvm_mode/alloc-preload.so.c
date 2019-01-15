/*  Preload file for overloading allocation functions
 *  Allows library functions that invoke allocation functions to work
 *  Allocates a prefix at the head of each heap chunk.
 */

#define _GNU_SOURCE

#include <stdio.h>
#include <dlfcn.h>
#include <sys/types.h>

#define LEN_SIZE sizeof(size_t)
#define P_SIZE sizeof(void *)
#define PREFIX_SIZE (LEN_SIZE + P_SIZE)

static unsigned char buffer[8192];

static void * (*real_malloc)(size_t)=NULL;
static void (*real_free)(void *) = NULL;
static void * (*real_calloc)(size_t, size_t) = NULL;
static void * (*real_reallocarray)(void *, size_t, size_t) = NULL;
static void * (*real_realloc)(void *, size_t) = NULL;

// Use constructor to get addresses immediately
static void __attribute__((constructor)) preload_init(void)
{
    // Put calloc first
    real_calloc = dlsym(RTLD_NEXT, "calloc");
    real_malloc = dlsym(RTLD_NEXT, "malloc");
    real_free = dlsym(RTLD_NEXT, "free");
    real_reallocarray = dlsym(RTLD_NEXT, "reallocarray");
    real_realloc = dlsym(RTLD_NEXT, "realloc");
    if (!real_malloc || !real_free || /*!real_calloc ||*/ !real_reallocarray || !real_realloc) {
        fprintf(stderr, "Error in `dlsym`: %s\n", dlerror());
        fflush(stderr);
    }
}

void * malloc(size_t size) {
    if(!real_malloc) {
        preload_init();
    }
    void * p = NULL;
    if (size) {
        p = real_malloc(size + PREFIX_SIZE);
        if (p != NULL){
            *((void **) p) = p;
            *((size_t *) (p + P_SIZE)) = size;
            p += PREFIX_SIZE;
        }
    }
    return p;
}

void free(void * ptr) {
    if (!real_free) {
        preload_init();
    }
    if (ptr) {
        ptr -= PREFIX_SIZE;
        ptr = *((void **) ptr);
        // If freeing temporary area from calloc
        if (ptr == 0xDEADBEEF) {
            return;
        }
        real_free(ptr);
    }
}

void * calloc(size_t nmemb, size_t size) {
    if (!nmemb || !size) {
        return NULL;
    }
    if (!real_calloc) {
        // Hack to avoid infinite recursion
        static void * base = buffer;
        void * ret = base;
        base += (nmemb * size) + PREFIX_SIZE;
        *((void **) ret) = 0xDEADBEEF;
        *((size_t *) ret + P_SIZE) = nmemb * size;
        return ret + PREFIX_SIZE;
    }
    // Get the number of items that would fit PREFIX_SIZE
    size_t prefix_n = PREFIX_SIZE / size ;
    if (prefix_n * size < PREFIX_SIZE) {
        prefix_n += 1;
    }
    void * p = real_calloc(nmemb + prefix_n, size);
    // Place the metadata after the array items.
    if (p) {
        *((void **) (p + (prefix_n * size) - PREFIX_SIZE)) = p;
        *((size_t *) (p + (prefix_n * size) - LEN_SIZE)) = nmemb * size;
        p += prefix_n * size;
    }
    return p;
}

void * reallocarray(void * ptr, size_t nmemb, size_t size) {
    if (real_reallocarray == NULL) {
        preload_init();
    }
    if (!ptr) {
        return calloc(nmemb, size);
    }
    // Unimplemented
    abort();
    void * p;
    p = real_reallocarray(ptr, nmemb, size);
    return p;
}

void * realloc(void * ptr, size_t size) {
    if (real_realloc == NULL) {
        preload_init();
    }
    if (!ptr) {
        return malloc(size);
    }
    if (!size) {
        free(ptr);
        return NULL;
    }
    // Retrieve crucial metadata
    size_t old_size = *((size_t *) (ptr - LEN_SIZE));
    void * base = *((void **)(ptr - PREFIX_SIZE));
    // If temporarily allocated by calloc()
    if (base == 0xDEADBEEF) {
        return NULL;
    }
    size_t prefix_size = ptr - base;
    
    void * p = real_realloc(base, size + prefix_size);
    if (p) {
        // Allocation successful, record new base and size
        *((size_t *) (p + prefix_size - LEN_SIZE)) = size;
        *((void **) (p + prefix_size - PREFIX_SIZE)) = p;
        p = p + prefix_size;
    }
    return p;
}
