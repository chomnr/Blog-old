<!DOCTYPE html>
<div class="flex justify-center items-center h-screen">
    <form class="w-full max-w-md bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
        <h1 class="text-xl font-bold mb-6 text-center">Login Form</h1>
        <div class="mb-4">
            <label class="block text-gray-700 font-bold mb-2" for="username">
                Username
            </label>
            <input class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Enter your username">
        </div>
        <div class="mb-6">
            <label class="block text-gray-700 font-bold mb-2" for="password">
                Password
            </label>
            <input class="appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline" id="password" type="password" placeholder="Enter your password">
        </div>
        <div class="flex items-center justify-between">
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button">
                Sign In
            </button>
        </div>
    </form>
</div>
