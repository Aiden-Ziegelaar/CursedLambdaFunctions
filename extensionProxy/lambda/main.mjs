export const handler = async (event) => {
  const response = {
    statusCode: 200,
    body: JSON.stringify({
      "message": `Hello ${event.name}`
    }),
  };
  return response;
};
  