
<script>
  let username = 'Testy';
  let email = 'Testy@gmail.com';
  let password = 'Doggy123!';
  let result = ''

  /**
   * @param {{ preventDefault: () => void; }} event
   */
  async function handleSubmit(event) {
    event.preventDefault();

    const formData = {
      username: username,
      email: email,
      password: password,
    };

    const response = await fetch('http://127.0.0.1:8000/api/user/create', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(formData),
    });

   // const data = await response.json();
   // console.log(data);
  }
</script>

<div class="flex flex-col justify-center items-center bg-[#fff] h-screen">
  <h3 class="text-center">LOGIN TO YOUR ACCOUNT</h3>
  <form class="space-y-5 w-full max-w-md bg-white rounded px-8 pt-6 pb-8 mb-4 text-sm" on:submit={handleSubmit}>
      <input bind:value={username} type="username" id="username" placeholder="Username" class="border-[1px] border-gray-400 p-1 min-w-[400px]" />
      <input bind:value={email} type="email" id="email" placeholder="Email Address" class="border-[1px] border-gray-400 p-1 min-w-[400px]" />
      <input bind:value={password} type="password" id="password" placeholder="Password" class="border-[1px] border-gray-400 p-1 min-w-[400px]" />
      <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="submit" >
          Register
      </button>
  </form>
</div>

<!--
<script>
    let username = 'doggy';c
    let email = 'hello@gmail.com';
    let password = 'Doggy123!das';
  
    /**
     * @param {{ preventDefault: () => void; }} event
     */
    async function handleSubmit(event) {
      event.preventDefault();
  
      const formData = {
        username: username,
        email: email,
        password: password,
      };
  
      const response = await fetch('http://127.0.0.1:8000/api/user/create', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(formData),
      });
  
      const data = await response.json();
      console.log(data);
    }
  </script>

<div class="flex justify-center items-center h-screen">
    <form  class="w-full max-w-md bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4" on:submit={handleSubmit}>
        <h1 class="text-xl font-bold mb-6 text-center">Register Form</h1>
        <div class="mb-4">
            <label class="block text-gray-700 font-bold mb-2" for="username">
                Username or Email
            </label>
            <input class="appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline" type="text" id="username" bind:value={username} />
        </div>
        <div class="mb-6">
            <label class="block text-gray-700 font-bold mb-2" for="email">
                Email
            </label>
            <input type="email" id="email" bind:value={email} />
            <label class="block text-gray-700 font-bold mb-2" for="password">
                Password
            </label>
            <input type="password" id="password" bind:value={password} />
        </div>
        <div class="flex items-center justify-between">
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="submit" >
                Sign In
            </button>
        </div>
    </form>
</div>
-->