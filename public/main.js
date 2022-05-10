console.log("test");

function init() {
  const input = document.getElementById("upload");
  const fireReader = new FileReader();
  fireReader.onloadend = () => {
    const base64 = fireReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );
    console.log(input.files[0]);
    console.log("base 64 of an image: ", base64);
  };
  input.addEventListener("change", () => {
    fireReader.readAsDataURL(input?.files[0]);
  });
}

init();

