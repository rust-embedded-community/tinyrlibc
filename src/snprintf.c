/**
 * Rudimentary C library implementation of `snprintf` and `vsnprintf`.
 *
 * `snprintf` takes a variable number of arguments and that isn't supported in
 * Rust. So, instead, here is a really basic `snprintf` implementation, which
 * is enough for Nordic Semiconductor's `libbsd` to link and run (at least for
 * the parts we've tested so far).
 *
 * Copyright (c) 42 Technology, 2019.
 * Licensed under the Blue Oak Model Licence 1.0.0
 */

/* ======================================================================== *
 *
 * System Includes
 *
 * ======================================================================== */
#include <string.h>
#include <stdbool.h>
#include <stdarg.h>
#include <stdint.h>

/* ======================================================================== *
 *
 * Private Function Declarations
 *
 * ======================================================================== */

static void write_output(
   char out, char* restrict str, size_t max, size_t* written );

static char upcase(char c);

/* ======================================================================== *
 *
 * Macros
 *
 * ======================================================================== */

/**
 * The longest number we can print is:
 *
 * -9223372036854775808
 */
#define MAXIMUM_NUMBER_LENGTH 21

/* ======================================================================== *
 *
 * External Function Declarations
 *
 * ======================================================================== */

/**
 * This is provided by `itoa.rs`. It converts a signed number to a string in the
 * specified radix.
 */
extern int32_t itoa(int64_t i, char* s, size_t s_len, uint8_t radix);

/**
 * This is provided by `itoa.rs`. It converts an unsigned number to a string
 * in the specified radix.
 */
extern int32_t utoa(uint64_t i, char* s, size_t s_len, uint8_t radix);

/* ======================================================================== *
 *
 * Public Function Definitions
 *
 * ======================================================================== */

/**
 * vsnprintf - string formatting with a `va_list` into a fixed sized buffer.
 *
 * Supports:
 *
 * - d/i (decimal signed integer)
 * - u (decimal unsigned integer)
 * - x (hexadecimal unsigned integer)
 * - c (char)
 * - s (null-terminated string)
 * - % (literal percent sign)
 * - qualifiers: l, ll, z
 *
 * Does not support:
 *
 * - p (none)
 * - p (pointer)
 * - o (octal)
 * - e (scientific notation with a lowercase e)
 * - E (scientific notation with a capital E)
 * - f (decimal floating point)
 * - g (the shorter of %e and %f)
 * - G (the shorter of %E and %f)
 * - qualifiers: L, width, precision, -, +, space-pad, zero-pad, etc
 *
 * @param str the output buffer to write to
 * @param size the size of the output buffer
 * @param fmt the format string
 * @param ap the variable number of arguments to render according to the
 *     format string
 * @return the number of characters written to `str`
 */
int vsnprintf(
   char* restrict str, size_t size, const char* restrict fmt, va_list ap )
{
   size_t written = 0;
   bool is_escape = false;
   int is_long = 0;
   bool is_size_t = false;
   while ( *fmt )
   {
      if ( is_escape )
      {
         is_escape = false;
         switch ( *fmt )
         {
         case 'z':
            if ( is_long || is_size_t ) {
               // not supported
               return -1;
            }
            is_size_t = true;
            is_escape = true;
            break;
         case 'l':
            is_long++;
            if ( is_long >= 3 ) {
               // not supported
               return -1;
            }
            is_escape = true;
            break;
         case 'u':
            if ( is_size_t )
            {
               // Render %zu
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               size_t su = va_arg( ap, size_t );
               utoa( su, s, sizeof(s), 10 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            else if ( is_long == 2 )
            {
               // Render %lu
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               unsigned long long ll = va_arg( ap, unsigned long long );
               utoa( ll, s, sizeof(s), 10 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            else if ( is_long == 1 )
            {
               // Render %lu
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               unsigned long l = va_arg( ap, unsigned long );
               utoa( l, s, sizeof(s), 10 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            else
            {
               // Render %u
               unsigned int i = va_arg( ap, unsigned int );
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               utoa( i, s, sizeof(s), 10 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            break;
         case 'x':
            if ( is_size_t )
            {
               // Render %zu
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               size_t su = va_arg( ap, size_t );
               utoa( su, s, sizeof(s), 16 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            else if ( is_long == 2 )
            {
               // Render %llu
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               unsigned long long ll = va_arg( ap, unsigned long long );
               utoa( ll, s, sizeof(s), 16 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            else if ( is_long == 1 )
            {
               // Render %lu
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               unsigned long l = va_arg( ap, unsigned long );
               utoa( l, s, sizeof(s), 16 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            else
            {
               // Render %u
               unsigned int i = va_arg( ap, unsigned int );
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               utoa( i, s, sizeof(s), 16 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            break;
         case 'X':
            if ( is_size_t )
            {
               // Render %zu
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               size_t su = va_arg( ap, size_t );
               utoa( su, s, sizeof(s), 16 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( upcase(*p), str, size, &written );
               }
            }
            else if ( is_long == 2 )
            {
               // Render %llu
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               unsigned long long ll = va_arg( ap, unsigned long long );
               utoa( ll, s, sizeof(s), 16 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( upcase(*p), str, size, &written );
               }
            }
            else if ( is_long == 1 )
            {
               // Render %lu
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               unsigned long l = va_arg( ap, unsigned long );
               utoa( l, s, sizeof(s), 16 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( upcase(*p), str, size, &written );
               }
            }
            else
            {
               // Render %u
               unsigned int i = va_arg( ap, unsigned int );
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               utoa( i, s, sizeof(s), 16 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( upcase(*p), str, size, &written );
               }
            }
            break;
         case 'i':
         case 'd':
            if ( is_long == 2 )
            {
               // Render %ld
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               signed long long ll = va_arg( ap, signed long long );
               itoa( ll, s, sizeof(s), 10 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            else if ( is_long == 1 )
            {
               // Render %ld
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               signed long l = va_arg( ap, signed long );
               itoa( l, s, sizeof(s), 10 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            else
            {
               // Render %d
               signed int i = va_arg( ap, signed int );
               char s[MAXIMUM_NUMBER_LENGTH] = { 0 };
               itoa( i, s, sizeof(s), 10 );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            break;
         case 'c':
            // Render %c
            {
               char c = (char) va_arg( ap, int );
               write_output( c, str, size, &written );
            }
            break;
         case 's':
            // Render %s
            {
               const char *s = va_arg( ap, const char* );
               for ( const char* p = s; *p != '\0'; p++ )
               {
                  write_output( *p, str, size, &written );
               }
            }
            break;
         case '%':
            write_output( '%', str, size, &written );
            break;
         default:
            /* Ignore unknown format code */
            break;
         }
         fmt++;
      }
      else
      {
         switch ( *fmt )
         {
         case '%':
            is_escape = true;
            is_long = 0;
            is_size_t = false;
            break;
         default:
            write_output( *fmt, str, size, &written );
            break;
         }
         fmt++;
      }
   }

   /* Add a terminating null (but don't count it) */
   if ( written < size )
   {
      str[written] = '\0';
   }

   return (int) written;
}

/**
 * snprintf - string formatting with `...` into a fixed sized buffer.
 *
 * Grabs the var-args as a `va_list` and calls `vsnprintf`.
 *
 * @param str the output buffer to write to
 * @param size the size of the output buffer
 * @param fmt the format string
 * @param ... the variable number of arguments to render according to the
 *     format string
 * @return the number of characters written to `str`
 */
int snprintf( char* restrict str, size_t size, const char* restrict fmt, ... )
{
   va_list ap;
   va_start( ap, fmt );
   int result = vsnprintf( str, size, fmt, ap );
   va_end( ap );
   return result;
}

/* ======================================================================== *
 *
 * Private Function Definitions
 *
 * ======================================================================== */

/**
 * write_output - Add a character to a bounded length buffer.
 *
 * If the character doesn't fit in the buffer, it is dropped, but the counter
 * is still incremented.
 *
 * @param out the character to write
 * @param str the buffer to write to
 * @param written pass in the number of characters in the buffer; is increased
 *     by one regardless of whether we wrote to the buffer
 * @param size the total size of `str`
 */
static void write_output(
   char out, char* restrict str, size_t size, size_t* written )
{
   // Check if it fits
   if ( *written < size )
   {
      // Write the character
      str[*written] = out;
      *written = *written + 1;
   }
   else
   {
      *written = *written + 1;
   }
}

/**
 * Converts 'a'..'z' to 'A'..'Z', leaving all other characters unchanged.
 */
static char upcase(char c) {
   if (( c >= 'a' ) && ( c <= 'z' ))
   {
      return (c - 'a') + 'A';
   }
   else
   {
      return c;
   }
}

/* ======================================================================== *
 *
 * End of File
 *
 * ======================================================================== */
