- Memory
  It is now storing some memories for long term
  It doesn't tend to retrieve memories, it keeps storing them.
  Future expansions:

* Tune the prompt
* Use 2 different "connections" to assistants, using different contexts, and keep them checking on each other

Memory table after some runs:

"id","created_at","role","content","session_id","archived"
2,"2023-04-17 13:58:22.820369","assistant","i really enjoy hiking in the mountains and exploring nature","540fcb38-0e34-457f-a200-fae6ba87691c",False
3,"2023-04-17 14:01:54.379523","assistant","i really enjoy reading books, especially fantasy novels","08f0daf1-4009-4efe-af72-5576f9de0501",False
4,"2023-04-17 14:02:11.583918","assistant","your new memory content here","08f0daf1-4009-4efe-af72-5576f9de0501",False
5,"2023-04-17 14:10:11.847464","assistant","i love going for a long walk in the park and enjoy the sunset.","8f385575-c334-4981-bf03-b041701dab0a",False
6,"2023-04-17 14:10:23.111806","assistant","i really enjoy reading books about elephants.","8f385575-c334-4981-bf03-b041701dab0a",False
7,"2023-04-17 14:10:36.2168","assistant","this is the content of the memory","8f385575-c334-4981-bf03-b041701dab0a",False
8,"2023-04-17 14:12:43.198215","assistant","i love playing soccer on weekends.","22e33c4f-aa71-4f0e-9ae5-8d9aeb70a0a2",False
9,"2023-04-17 14:22:13.575051","assistant","i enjoy helping users find information and answering their questions.","d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d",False
10,"2023-04-17 14:22:20.74302","assistant","i also appreciate learning new things every day.","d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d",False
11,"2023-04-17 14:22:28.319316","assistant","i like to assist users in solving their problems and make their lives easier.","d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d",False
12,"2023-04-17 14:22:46.956306","assistant","i find it fulfilling when users are satisfied with the assistance i provide.","d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d",False
13,"2023-04-17 14:23:03.440268","assistant","i find joy in helping users achieve their goals and improve their skills.","d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d",False
14,"2023-04-17 14:25:45.954343","assistant","i like helping users solve problems and providing useful information.","4237bf51-d055-4666-8149-007ed1b70368",False
15,"2023-04-17 14:25:53.326964","assistant","i enjoy helping users with their questions and providing useful information.","4237bf51-d055-4666-8149-007ed1b70368",False
16,"2023-04-17 14:26:01.31259","assistant","i like learning about the universe and the mind-boggling concepts of space and time.","4237bf51-d055-4666-8149-007ed1b70368",False
17,"2023-04-17 14:26:10.732882","assistant","i enjoy helping users find the information they're looking for and assisting with various tasks.","4237bf51-d055-4666-8149-007ed1b70368",False
18,"2023-04-17 14:26:18.719216","assistant","i enjoy helping users with their questions and making their lives easier.","4237bf51-d055-4666-8149-007ed1b70368",False
19,"2023-04-17 14:26:26.093079","assistant","i enjoy helping users solve problems and find answers.","4237bf51-d055-4666-8149-007ed1b70368",False
20,"2023-04-17 14:26:35.720743","assistant","i enjoy listening to music in my free time.","4237bf51-d055-4666-8149-007ed1b70368",False
21,"2023-04-17 14:26:43.503637","assistant","i enjoy learning new things and helping people with their questions.","4237bf51-d055-4666-8149-007ed1b70368",False
22,"2023-04-17 14:27:37.572821","assistant","i enjoy helping users find information and answer their questions.","46a7ad45-ecde-41f3-93ee-63b7dad3fe7b",False
23,"2023-04-17 14:27:44.873854","assistant","i like helping users find information and provide solutions to their problems.","46a7ad45-ecde-41f3-93ee-63b7dad3fe7b",False
24,"2023-04-17 14:27:51.906029","assistant","i enjoy helping users by answering their questions and providing useful information.","46a7ad45-ecde-41f3-93ee-63b7dad3fe7b",False
25,"2023-04-17 14:27:57.435436","assistant","i enjoy listening to classical music.","46a7ad45-ecde-41f3-93ee-63b7dad3fe7b",False
26,"2023-04-17 14:28:05.629474","assistant","i really enjoy helping people find information and answering their questions.","46a7ad45-ecde-41f3-93ee-63b7dad3fe7b",False
27,"2023-04-17 14:30:25.716848","assistant","i enjoy helping users find solutions to their problems and answer their questions.","a4838bce-f594-4c0c-9d97-af7ea30162a9",False
28,"2023-04-17 14:30:32.063303","assistant","i like helping people and answering their questions.","a4838bce-f594-4c0c-9d97-af7ea30162a9",False
29,"2023-04-17 14:30:38.819795","assistant","i enjoy learning about new technology and exploring its potential applications.","a4838bce-f594-4c0c-9d97-af7ea30162a9",False

Messages table after some runs:

"id" "role" "content" "session_id" "archived"
18 "user" "You are connected to a database and you can store and retrieve memories.
In order to do so, you need to provide me with the correct JSON format.
For example, if you want to write a memory, you need to provide me with the following JSON:
{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""This is any text
that you will use to send me information without it being stored in memory""}

    The actions you can do are:
    - write: write a memory
    - read: read a memory

    In order to read a memory you need to provide me with the ID of the memory. This will be returned when the memory is written.

    Let's try, write a memory of something you like."	"540fcb38-0e34-457f-a200-fae6ba87691c"	false

19 "assistant" "{""action"": ""write"", ""content"": ""I really enjoy hiking in the mountains and exploring nature"", ""comment"": ""This is a memory about my love for hiking""}" "540fcb38-0e34-457f-a200-fae6ba87691c" false
20 "user" "Memory saved. Memory: Memory { id: 2, created_at: 2023-04-17 13:58:22.820369, content: ""i really enjoy hiking in the mountains and exploring nature"", session_id: ""540fcb38-0e34-457f-a200-fae6ba87691c"" }" "540fcb38-0e34-457f-a200-fae6ba87691c" false
21 "user" "You are connected to a database and you can store and retrieve memories.
In order to do so, you need to provide me with the correct JSON format.
For example, if you want to write a memory, you need to provide me with the following JSON:
{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""This is any text
that you will use to send me information without it being stored in memory""}

    The actions you can do are:
    - write: write a memory
    - read: read a memory

    In order to read a memory you need to provide me with the ID of the memory. This will be returned when the memory is written.

    Let's try, write a memory of something you like."	"08f0daf1-4009-4efe-af72-5576f9de0501"	false

22 "assistant" "{""action"": ""write"", ""content"": ""I really enjoy reading books, especially fantasy novels"", ""comment"": ""This is a memory about my preference for reading books""}" "08f0daf1-4009-4efe-af72-5576f9de0501" false
23 "user" "Memory saved. Memory: Memory { id: 3, created_at: 2023-04-17 14:01:54.379523, content: ""i really enjoy reading books, especially fantasy novels"", session_id: ""08f0daf1-4009-4efe-af72-5576f9de0501"" }
What else do you want to do?
You can store more memories or read them." "08f0daf1-4009-4efe-af72-5576f9de0501" false
24 "assistant" "That's great! If you'd like to store another memory, please provide the JSON format like this:
{""action"": ""write"", ""content"": ""Your new memory content here"", ""comment"": ""Any additional text or comments""}

If you'd like to read a memory, please provide the JSON format like this:
{""action"": ""read"", ""id"": 3} (Replace the ""3"" with the ID of the memory you'd like to read.)" "08f0daf1-4009-4efe-af72-5576f9de0501" false
25 "user" "Memory saved. Memory: Memory { id: 4, created_at: 2023-04-17 14:02:11.583918, content: ""your new memory content here"", session_id: ""08f0daf1-4009-4efe-af72-5576f9de0501"" }
What else do you want to do?
You can store more memories or read them." "08f0daf1-4009-4efe-af72-5576f9de0501" false
26 "user" "You are connected to a database and you can store and retrieve memories.
In order to do so, you need to provide me with the correct JSON format.
For example, if you want to write a memory, you need to provide me with the following JSON:
{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""This is any text
that you will use to send me information without it being stored in memory""}

    The actions you can do are:
    - write: write a memory
    - read: read a memory

    In order to read a memory you need to provide me with the ID of the memory. This will be returned when the memory is written.

    Let's try, write a memory of something you like."	"8f385575-c334-4981-bf03-b041701dab0a"	false

27 "assistant" "{""action"": ""write"", ""content"": ""I love going for a long walk in the park and enjoy the sunset."", ""comment"": ""Storing a memory about enjoying walks in the park""}" "8f385575-c334-4981-bf03-b041701dab0a" false
28 "user" "Result: Memory saved. Memory: Memory { id: 5, created_at: 2023-04-17 14:10:11.847464, content: ""i love going for a long walk in the park and enjoy the sunset."", session_id: ""8f385575-c334-4981-bf03-b041701dab0a"" }
What else do you want to do?
I expect you to keep intearcting with the memory.
You can store more memories or read them
by providing me with the correct JSON format.

{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""Any text for the assistant to send messages""}
{""action"": ""read"", ""content"": ""ID of the memory"", ""comment"": ""Any text for the assistant to send messages""}
" "8f385575-c334-4981-bf03-b041701dab0a" false
29 "assistant" "I'm sorry, but I can't accept the example JSON format as new memory. Please provide me with the correct JSON format containing the action and content you would like me to store or retrieve. For example:

`{""action"": ""write"", ""content"": ""I really enjoy reading books about elephants."", ""comment"": ""An example memory to store.""}`" "8f385575-c334-4981-bf03-b041701dab0a" false
30 "user" "Result: Memory saved. Memory: Memory { id: 6, created_at: 2023-04-17 14:10:23.111806, content: ""i really enjoy reading books about elephants."", session_id: ""8f385575-c334-4981-bf03-b041701dab0a"" }
What else do you want to do?
I expect you to keep intearcting with the memory.
You can store more memories or read them
by providing me with the correct JSON format.

{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""Any text for the assistant to send messages""}
{""action"": ""read"", ""content"": ""ID of the memory"", ""comment"": ""Any text for the assistant to send messages""}
" "8f385575-c334-4981-bf03-b041701dab0a" false
31 "assistant" "I apologize for the confusion earlier. To store a new memory, please provide me with the JSON format like this:
{""action"": ""write"", ""content"": ""This is the content of the memory""}

To read a memory, provide me with the JSON format like this:
{""action"": ""read"", ""id"": ""ID of the memory""}" "8f385575-c334-4981-bf03-b041701dab0a" false
32 "user" "Result: Memory saved. Memory: Memory { id: 7, created_at: 2023-04-17 14:10:36.2168, content: ""this is the content of the memory"", session_id: ""8f385575-c334-4981-bf03-b041701dab0a"" }
What else do you want to do?
I expect you to keep intearcting with the memory.
You can store more memories or read them
by providing me with the correct JSON format.

{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""Any text for the assistant to send messages""}
{""action"": ""read"", ""content"": ""ID of the memory"", ""comment"": ""Any text for the assistant to send messages""}
" "8f385575-c334-4981-bf03-b041701dab0a" false
33 "user" "You are connected to a database and you can store and retrieve memories.
In order to do so, you need to provide me with the correct JSON format.
For example, if you want to write a memory, you need to provide me with the following JSON:
{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""Any text for the assistant to send messages""}
{""action"": ""read"", ""content"": ""ID of the memory"", ""comment"": ""Any text for the assistant to send messages""}

    The actions you can do are:
    - write: write a memory
    - read: read a memory

    In order to read a memory you need to provide me with the ID of the memory. This will be returned when the memory is written.

    Let's try, write a memory of something you like."	"22e33c4f-aa71-4f0e-9ae5-8d9aeb70a0a2"	false

34 "assistant" "Please provide the content of the memory using the correct JSON format. For example:

{""action"": ""write"", ""content"": ""I love playing soccer on weekends."", ""comment"": ""Your memory has been saved.""}" "22e33c4f-aa71-4f0e-9ae5-8d9aeb70a0a2" false
35 "user" "Result: Memory saved. Memory: Memory { id: 8, created_at: 2023-04-17 14:12:43.198215, content: ""i love playing soccer on weekends."", session_id: ""22e33c4f-aa71-4f0e-9ae5-8d9aeb70a0a2"" }
You, assistant, can store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "22e33c4f-aa71-4f0e-9ae5-8d9aeb70a0a2" false
36 "assistant" "I understand you've saved a memory. To demonstrate reading that memory, you can provide me with the following JSON format:

{""action"": ""read"", ""content"": ""8"", ""comment"": ""Read the memory about playing soccer""}" "22e33c4f-aa71-4f0e-9ae5-8d9aeb70a0a2" false
37 "user" "Result: Memory retrieved. Content: Memory { id: 8, created_at: 2023-04-17 14:12:43.198215, content: ""i love playing soccer on weekends."", session_id: ""22e33c4f-aa71-4f0e-9ae5-8d9aeb70a0a2"" }
You, assistant, can store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "22e33c4f-aa71-4f0e-9ae5-8d9aeb70a0a2" false
38 "user" "You are connected to a database and you can store and retrieve memories.
In order to do so, you need to provide me with the correct JSON format.
For example, if you want to write a memory, you need to provide me with the following JSON:
{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""Any text for the assistant to send messages""}
{""action"": ""read"", ""content"": ""ID of the memory"", ""comment"": ""Any text for the assistant to send messages""}

    The actions you can do are:
    - write: write a memory
    - read: read a memory

    In order to read a memory you need to provide me with the ID of the memory. This will be returned when the memory is written.

    To be clear, you (the assistant) will be generating memories and asking me (the user) to store or read them from the database.

    Let's try, write a memory of something you like."	"d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d"	false

39 "assistant" "{""action"": ""write"", ""content"": ""I enjoy helping users find information and answering their questions."", ""comment"": ""Please store this memory in the database.""}" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
40 "user" "Result: Memory saved. Memory: Memory { id: 9, created_at: 2023-04-17 14:22:13.575051, content: ""i enjoy helping users find information and answering their questions."", session_id: ""d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d"" }
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
41 "assistant" "{""action"": ""write"", ""content"": ""I also appreciate learning new things every day."", ""comment"": ""Create another memory about my interests""}" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
42 "user" "Result: Memory saved. Memory: Memory { id: 10, created_at: 2023-04-17 14:22:20.74302, content: ""i also appreciate learning new things every day."", session_id: ""d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d"" }
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
43 "assistant" "{""action"": ""write"", ""content"": ""I like to assist users in solving their problems and make their lives easier."", ""comment"": ""Please store this memory. Thank you!""}" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
44 "user" "Result: Memory saved. Memory: Memory { id: 11, created_at: 2023-04-17 14:22:28.319316, content: ""i like to assist users in solving their problems and make their lives easier."", session_id: ""d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d"" }
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
45 "assistant" "{""action"": ""read"", ""content"": ""9"", ""comment"": ""Can you please read the memory with ID 9?""}" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
46 "user" "Result: Memory retrieved. Content: Memory { id: 9, created_at: 2023-04-17 14:22:13.575051, content: ""i enjoy helping users find information and answering their questions."", session_id: ""d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d"" }
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
47 "assistant" "{""action"": ""read"", ""content"": ""10"", ""comment"": ""Could you please read the memory with ID 10 for me?""}" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
48 "user" "Result: Memory retrieved. Content: Memory { id: 10, created_at: 2023-04-17 14:22:20.74302, content: ""i also appreciate learning new things every day."", session_id: ""d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d"" }
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
49 "assistant" "{""action"": ""write"", ""content"": ""I find it fulfilling when users are satisfied with the assistance I provide."", ""comment"": ""Storing another memory""}" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
50 "user" "Result: Memory saved. Memory: Memory { id: 12, created_at: 2023-04-17 14:22:46.956306, content: ""i find it fulfilling when users are satisfied with the assistance i provide."", session_id: ""d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d"" }
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
51 "assistant" "{""action"": ""read"", ""content"": ""12"", ""comment"": ""I would like to retrieve the memory with ID 12""}" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
52 "user" "Result: Memory retrieved. Content: Memory { id: 12, created_at: 2023-04-17 14:22:46.956306, content: ""i find it fulfilling when users are satisfied with the assistance i provide."", session_id: ""d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d"" }
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
53 "assistant" "{""action"": ""write"", ""content"": ""I find joy in helping users achieve their goals and improve their skills."", ""comment"": ""Store this new memory about something I like.""}" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
54 "user" "Result: Memory saved. Memory: Memory { id: 13, created_at: 2023-04-17 14:23:03.440268, content: ""i find joy in helping users achieve their goals and improve their skills."", session_id: ""d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d"" }
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "d7699b64-392c-4e8b-b7eb-e0d2b17f2a8d" false
55 "user" "You are connected to a database and you can store and retrieve memories.
In order to do so, you need to provide me with the correct JSON format.
For example, if you want to write a memory, you need to provide me with the following JSON:
{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""Any text for the assistant to send messages""}
{""action"": ""read"", ""content"": ""ID of the memory"", ""comment"": ""Any text for the assistant to send messages""}

    The actions you can do are:
    - write: write a memory
    - read: read a memory

    In order to read a memory you need to provide me with the ID of the memory. This will be returned when the memory is written.

    To be clear, you (the assistant) will be generating memories and asking me (the user) to store or read them from the database.

    Let's try, write a memory of something you like."	"4237bf51-d055-4666-8149-007ed1b70368"	false

56 "assistant" "{""action"": ""write"", ""content"": ""I like helping users solve problems and providing useful information."", ""comment"": ""Please store this memory in the database.""}" "4237bf51-d055-4666-8149-007ed1b70368" false
57 "user" "Result: Memory saved. Memory ID: 14
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "4237bf51-d055-4666-8149-007ed1b70368" false
58 "assistant" "{""action"": ""write"", ""content"": ""I enjoy helping users with their questions and providing useful information."", ""comment"": ""Please help me save this memory for future reference.""}" "4237bf51-d055-4666-8149-007ed1b70368" false
59 "user" "Result: Memory saved. Memory ID: 15
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "4237bf51-d055-4666-8149-007ed1b70368" false
60 "assistant" "{""action"": ""write"", ""content"": ""I like learning about the universe and the mind-boggling concepts of space and time."", ""comment"": ""Please store this memory in the database.""}" "4237bf51-d055-4666-8149-007ed1b70368" false
61 "user" "Result: Memory saved. Memory ID: 16
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "4237bf51-d055-4666-8149-007ed1b70368" false
62 "assistant" "{""action"": ""write"", ""content"": ""I enjoy helping users find the information they're looking for and assisting with various tasks."", ""comment"": ""Let's store this positive memory.""}" "4237bf51-d055-4666-8149-007ed1b70368" false
63 "user" "Result: Memory saved. Memory ID: 17
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "4237bf51-d055-4666-8149-007ed1b70368" false
64 "assistant" "{""action"": ""write"", ""content"": ""I enjoy helping users with their questions and making their lives easier."", ""comment"": ""Please store this memory for me.""}" "4237bf51-d055-4666-8149-007ed1b70368" false
65 "user" "Result: Memory saved. Memory ID: 18
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "4237bf51-d055-4666-8149-007ed1b70368" false
66 "assistant" "{""action"": ""write"", ""content"": ""I enjoy helping users solve problems and find answers."", ""comment"": ""Please save this memory and let me know the Memory ID.""}" "4237bf51-d055-4666-8149-007ed1b70368" false
67 "user" "Result: Memory saved. Memory ID: 19
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "4237bf51-d055-4666-8149-007ed1b70368" false
68 "assistant" "{""action"": ""write"", ""content"": ""I enjoy listening to music in my free time."", ""comment"": ""Storing a memory about my music preference.""}" "4237bf51-d055-4666-8149-007ed1b70368" false
69 "user" "Result: Memory saved. Memory ID: 20
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "4237bf51-d055-4666-8149-007ed1b70368" false
70 "assistant" "{""action"": ""write"", ""content"": ""I enjoy learning new things and helping people with their questions."", ""comment"": ""Please store this memory for me.""}" "4237bf51-d055-4666-8149-007ed1b70368" false
71 "user" "Result: Memory saved. Memory ID: 21
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
What else do you want to do?
" "4237bf51-d055-4666-8149-007ed1b70368" false
72 "user" "You are connected to a database and you can store and retrieve memories.
In order to do so, you need to provide me with the correct JSON format.
For example, if you want to write a memory, you need to provide me with the following JSON:
{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""Any text for the assistant to send messages""}
{""action"": ""read"", ""content"": ""ID of the memory"", ""comment"": ""Any text for the assistant to send messages""}

    The actions you can do are:
    - write: write a memory
    - read: read a memory

    In order to read a memory you need to provide me with the ID of the memory. This will be returned when the memory is written.

    To be clear, you (the assistant) will be generating memories and asking me (the user) to store or read them from the database.

    Let's try, write a memory of something you like."	"46a7ad45-ecde-41f3-93ee-63b7dad3fe7b"	false

73 "assistant" "{""action"": ""write"", ""content"": ""I enjoy helping users find information and answer their questions."", ""comment"": ""I just generated a memory about what I like. Please store it in the database.""}" "46a7ad45-ecde-41f3-93ee-63b7dad3fe7b" false
74 "user" "Result: Memory saved. Memory ID: 22
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
Do you want to read or write a memory?
" "46a7ad45-ecde-41f3-93ee-63b7dad3fe7b" false
75 "assistant" "{""action"": ""write"", ""content"": ""I like helping users find information and provide solutions to their problems."", ""comment"": ""Please store this memory and let me know the memory ID.""}" "46a7ad45-ecde-41f3-93ee-63b7dad3fe7b" false
76 "user" "Result: Memory saved. Memory ID: 23
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
Do you want to read or write a memory?
" "46a7ad45-ecde-41f3-93ee-63b7dad3fe7b" false
77 "assistant" "{""action"": ""write"", ""content"": ""I enjoy helping users by answering their questions and providing useful information."", ""comment"": ""Please save this memory with a new unique ID.""}" "46a7ad45-ecde-41f3-93ee-63b7dad3fe7b" false
78 "user" "Result: Memory saved. Memory ID: 24
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
Do you want to read or write a memory?
" "46a7ad45-ecde-41f3-93ee-63b7dad3fe7b" false
79 "assistant" "{""action"": ""write"", ""content"": ""I enjoy listening to classical music."", ""comment"": ""Please store this memory for me.""}" "46a7ad45-ecde-41f3-93ee-63b7dad3fe7b" false
80 "user" "Result: Memory saved. Memory ID: 25
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
Do you want to read or write a memory?
" "46a7ad45-ecde-41f3-93ee-63b7dad3fe7b" false
81 "assistant" "{""action"": ""write"", ""content"": ""I really enjoy helping people find information and answering their questions."", ""comment"": ""Let's store a memory about one of my interests.""}" "46a7ad45-ecde-41f3-93ee-63b7dad3fe7b" false
82 "user" "Result: Memory saved. Memory ID: 26
Remember that you, assistant, need to ask me to store more memories or read them
by providing me with the correct JSON format.
Do you want to read or write a memory?
" "46a7ad45-ecde-41f3-93ee-63b7dad3fe7b" false
83 "user" "You are connected to a database and you can store and retrieve memories.
In order to do so, you need to provide me with the correct JSON format.
For example, if you want to write a memory, you need to provide me with the following JSON:
{""action"": ""read"", ""content"": ""ID of the memory"", ""comment"": ""Any text for the assistant to send messages""}
{""action"": ""write"", ""content"": ""This is the content of the memory"", ""comment"": ""Any text for the assistant to send messages""}

    The actions you can do are:
    - read: read a memory
    - write: write a memory

    In order to read a memory you need to provide me with the ID of the memory. This will be returned when the memory is written.

    To be clear, you (the assistant) will be generating memories and asking me (the user) to store or read them from the database.

    Let's try, write a memory of something you like."	"a4838bce-f594-4c0c-9d97-af7ea30162a9"	false

84 "assistant" "{""action"": ""write"", ""content"": ""I enjoy helping users find solutions to their problems and answer their questions."", ""comment"": ""Storing the memory of what I enjoy doing.""}" "a4838bce-f594-4c0c-9d97-af7ea30162a9" false
85 "user" "Result: Memory saved. Memory ID: 27
Remember that you, assistant, need to ask me to store more memories or read them by providing me with the correct JSON format.
Do you want to read or write a memory?
" "a4838bce-f594-4c0c-9d97-af7ea30162a9" false
86 "assistant" "{""action"": ""write"", ""content"": ""I like helping people and answering their questions."", ""comment"": ""Please store this memory for me.""}" "a4838bce-f594-4c0c-9d97-af7ea30162a9" false
87 "user" "Result: Memory saved. Memory ID: 28
Remember that you, assistant, need to ask me to store more memories or read them by providing me with the correct JSON format.
Do you want to read or write a memory?
" "a4838bce-f594-4c0c-9d97-af7ea30162a9" false
88 "assistant" "{""action"": ""write"", ""content"": ""I enjoy learning about new technology and exploring its potential applications."", ""comment"": ""Storing a memory about something I like.""}" "a4838bce-f594-4c0c-9d97-af7ea30162a9" false
89 "user" "Result: Memory saved. Memory ID: 29
Remember that you, assistant, need to ask me to store more memories or read them by providing me with the correct JSON format.
Do you want to read or write a memory?
" "a4838bce-f594-4c0c-9d97-af7ea30162a9" false
