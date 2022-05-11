console.log("test");

async function init() {
  let rustApp = null;
  try {
    rustApp = await import("../pkg");
  } catch (error) {
    console.error(error);
    return;
  }
  console.log(rustApp);
  const input = document.getElementById("upload");
  const fireReader = new FileReader();
  fireReader.onloadend = () => {
    const base64 = fireReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );
    let imgDataUrl = rustApp.grayScale(base64);
    document.getElementById("new-img").setAttribute("src", imgDataUrl);
  };
  input.addEventListener("change", () => {
    fireReader.readAsDataURL(input?.files[0]);
  });
}

init();

