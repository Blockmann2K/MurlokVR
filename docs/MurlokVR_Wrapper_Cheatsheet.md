# 🦀 Wrapper Writing Cheat Sheet — Rust + Unsafe FFI

> Written With the Intention of Wrapping the Windows Shared Memory API for the MurlokVR IPC Bridge.

---

## 01 — 🧠 The Core Philosophy
- Unsafe Code Should Be **Rare, Isolated, and Auditable.** Your Wrapper Is the Wall Between Unsafe and the Rest of Your Codebase.
- Everything *Inside* the Wrapper Can Be Unsafe. Everything *Outside* Should Feel Normal.

---

## 02 — 🏗️ Struct Design
- Your Wrapper Struct Should **Own** the Resource — Handle, Pointer, Etc.
- Make All Fields **Private** — Callers Should Never Touch the Raw Handle Directly.
- Consider Whether Your Type Should Implement `Send` and/or `Sync` — Windows Handles Are Generally *Not* Automatically Safe to Share Across Threads. Both Implementations Will Be `unsafe` — You Are Making a Promise the Compiler Cannot Verify. Be Deliberate.

---

## 03 — 🛡️ Always Implement `Drop`
- `Drop` Is Your Safety Net — It Guarantees Cleanup Even on Panics or Early Returns.
- Call Your Cleanup Functions (`CloseHandle`, `UnmapViewOfFile`, Etc.) Inside `drop()`.
- Never Rely on the Caller to Clean Up Manually.

---

## 04 — ⚠️ Error Handling
- **Never Ignore a Null Handle or Error Code** — Check Every Return Value From FFI Calls.
- Return `Result<T, E>` From Your Constructor, Not a Raw Value.
- Use `GetLastError` via `windows-sys` to Get Meaningful Error Info on Failure.

---

## 05 — 🚧 The `unsafe` Boundary Rule
- A Function That *Calls* Unsafe Code Can Itself Be **Safe** — as Long as You Have Upheld All Invariants the Unsafe Code Requires.
- If You *Cannot* Guarantee Safety for the Caller, Mark the Function `unsafe fn` and Document Why.

---

## 06 — ⏳ Lifetime & Ownership Clarity
- Who *Creates* the Shared Memory? Who Only *Opens* It? Consider **Separate Types** for Owner vs. Accessor.
- A Mapped Pointer Is Only Valid as Long as the Mapping Exists — Don't Let It Outlive the Wrapper.

---

## 07 — 👻 Null Pointer Semantics
- Invalid Handles Are Sometimes `null`, Sometimes `INVALID_HANDLE_VALUE` (-1) — **They Are Not the Same Thing.**
- Checking for the Wrong Sentinel Value Is a Silent Bug — Know Which Each Function Returns.
- `CreateFileMappingW` Returns `NULL` on Failure. `CreateFile` Returns `INVALID_HANDLE_VALUE` — Know Which Is Which!

---

## 08 — 🔤 String Encoding
- Windows Wide-String APIs (Anything Ending in `W`) Expect **UTF-16 Encoded, Null-Terminated** Strings.
- Rust's `&str` Is UTF-8 With No Null Terminator — Passing It Directly to a `W` Function Will Silently Produce Garbage or a Crash.
- Always Encode to UTF-16 and Append a Null Terminator Before Passing Any String Into a Windows API Call.

---

## 09 — 📝 Document Your Invariants
- Write a Short `// SAFETY:` Comment Above Every `unsafe` Block Explaining *Why It Is Safe.*
- This Is for Future-You During a Debugging Session. 🙂

---

## 10 — 📐 `#[repr(C)]` and Struct Layout
> ⚠️ *Especially Critical for MurlokVR — Every Struct Written to Shared Memory Must Cross the Rust / C++ Boundary Correctly.*

- Without `#[repr(C)]`, Rust Is Free to **Reorder or Pad Struct Fields** However It Likes at Compile Time — the C++ Side Will Silently Read Garbage With No Error or Warning.
- Every Struct That Crosses the Shared Memory Boundary **Must** Be Annotated With `#[repr(C)]` — No Exceptions.
- This Is Not Just a Best Practice — It Is the Contract Between the Two Processes.

---

## 11 — 🔢 Alignment & Padding
> ⚠️ *Especially Critical for MurlokVR — the Shared Memory Layout Mixes `f32` and `u64` Fields, Which Can Produce Unexpected Padding.*

- The Compiler May Insert **Silent Padding Bytes** Between Fields to Satisfy Alignment Requirements — Even With `#[repr(C)]`.
- A `u64` Field (Like the Heartbeat Timestamp) Requires 8-Byte Alignment — If It Follows a `f32`, the Compiler Will Insert **4 Bytes of Padding** Before It.
- If the C++ Side Does Not Match This Layout Exactly, Every Field After the Padding Will Be Read at the Wrong Offset.
- Always **Verify the Struct Layout** on Both Sides Is Identical Before Trusting the Shared Memory Data.

---

## 12 — 📖 Glossary

- 📨 `Send` — Safe to **Move** to Another Thread. Implement if Transferring Ownership Across Threads Is Valid for Your Type.
- 🔗 `Sync` — Safe to **Share a Reference** Across Threads Simultaneously. Implement if Multiple Threads Reading It at the Same Time Is Valid.
- 🔚 **Null-Terminated String** — A Sequence of Characters With a **Zero Value (`\0`) Appended at the End.** It Is How C and the Windows API Know Where a String Stops. Rust Strings Store Their Length Explicitly and Have No Such Terminator.
- 🔡 **UTF-16** — A Character Encoding Used by Windows Internally. Each Character Is Represented as One or More 16-Bit Values. Rust's Strings Are UTF-8 — the Two Are Not Directly Compatible.
- 🚩 **Sentinel Value** — A Special Value Used to Signal a Specific Condition, Like Failure. `NULL` and `INVALID_HANDLE_VALUE` Are Both Sentinel Values for Invalid Handles in the Windows API.
- ♻️ **RAII** — *Resource Acquisition Is Initialization.* Tie a Resource's Lifetime to a Struct's Lifetime — Acquire on Creation, Release on Drop. Rust's `Drop` Trait Is RAII.
- 🌉 **FFI** — *Foreign Function Interface.* The Mechanism That Allows Rust to Call Functions Written in Other Languages, Like the Windows API Written in C.
