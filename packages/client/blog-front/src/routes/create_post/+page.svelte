<!DOCTYPE html>

<div class="flex justify-center items-center h-screen">
    <form class="w-full max-w-md bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
        <h1 class="text-xl font-bold mb-6 text-center">Login Form</h1>
        <div class="mb-4">
            <label class="block text-gray-700 font-bold mb-2" for="post_title">
                Title
            </label>
            <input class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="post_title" type="text" placeholder="Title">
        </div>
        <div class="mb-6">
            <label class="block text-gray-700 font-bold mb-2" for="post_body">
                Body
            </label>
            <textarea
            class="appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="body" name="body" rows="5" placeholder="Enter thread body"></textarea>
        </div>
        <div class="flex items-center justify-between">
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button">
                Create Post
            </button>
        </div>
    </form>
</div>
