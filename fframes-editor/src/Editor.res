@genType.as("Editor") @react.component
let make = () => {
  <div className="w-screen h-screen bg-gray-900">
    <div className="overflow-auto grid grid-cols-7 w-full h-2/3 ">
      <div className="col-span-2  h-full overflow-auto flex flex-col p-6">
        <h1 className="text-4xl font-medium text-white"> {React.string("Test Video")} </h1>
      </div>
      <div className="col-span-5 bg-black" />
    </div>
    <div className="h-1/3 shadow-lg w-screen bg-gray-800"> <Timeline /> </div>
  </div>
}
