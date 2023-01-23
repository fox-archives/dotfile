#include <stdio.h> // printf()
#include <sys/utsname.h> // uname()

int main() {
        int ret; // stores the return value of uname()
        struct utsname utsname; // stores the data returned by uname()
        struct utsname *utsname_ptr = &utsname; // pointer to the struct holding the data returned by uname()

        ret = uname(utsname_ptr); // calls uname() on utsname_ptr and stores its return value in ret

        /* prints the fields of utsname */

        printf("%s\n", utsname.sysname);
        printf("%s\n", utsname.nodename);
        printf("%s\n", utsname.release);
        printf("%s\n", utsname.version);
        printf("%s\n", utsname.machine);

        /* returns the return value of uname() */

        return(ret);
}
