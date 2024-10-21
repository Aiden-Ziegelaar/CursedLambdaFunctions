export const handler = async (event) => {
    let extension_response = await fetch("http://localhost:1337/hello/world", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ name: event.name }),
    })

    const response = {
      statusCode: 200,
      body: await extension_response.text(),
    };
    return response;
  };
  