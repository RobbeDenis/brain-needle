section .data
tape times 30000 db 0
section .text
global _start
_start:
lea rsi, [rel tape]
mov [rsi], 0
add byte [rsi], 10
cmp byte [rsi], 0
LS_1:
jz LE_1
add rsi, 1
add byte [rsi], 1
add rsi, 1
add byte [rsi], 3
add rsi, 1
add byte [rsi], 7
add rsi, 1
add byte [rsi], 10
sub rsi, 4
sub byte [rsi], 1
jmp LS_1
LE_1:
add rsi, 3
add byte [rsi], 2
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add rsi, 1
add byte [rsi], 1
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add byte [rsi], 7
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add byte [rsi], 3
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
sub rsi, 2
add byte [rsi], 2
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add rsi, 1
add byte [rsi], 15
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add rsi, 1
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add byte [rsi], 3
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
sub byte [rsi], 6
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
sub byte [rsi], 8
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
mov rax, 60
xor rdi, rdi
syscall