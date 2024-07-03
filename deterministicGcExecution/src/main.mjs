export const handler = async (event) => {
    global.gc();
    const response = {
        statusCode: 200,
        body: JSON.stringify({
        "message": `Hello ${event.name}`
        }),
    };
    return response;
};