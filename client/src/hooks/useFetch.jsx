import { useEffect, useState } from "react";

const API_KEY = 'R9frzoWNSKe7ImoLRylybyCyvxrBbpjJzvRy5sY1X3s'; // Your Unsplash access key

const useFetch = ({ keyword }) => {
  const [imageUrl, setImageUrl] = useState("");

  const fetchImages = async () => {
    try {
      const response = await fetch(`https://api.unsplash.com/search/photos?query=${keyword}&per_page=1&client_id=${API_KEY}`);

      const { results } = await response.json();

      if (results.length > 0) {
        setImageUrl(results[0]?.urls?.regular);
      } else {
        // Set a default image URL if no results are found
        setImageUrl('https://example.com/default-image.jpg');
      }
    } catch (error) {
      // Handle error or set a default image URL
      setImageUrl('https://example.com/default-image.jpg');
    }
  };

  useEffect(() => {
    if (keyword) fetchImages();
  }, [keyword]);

  return imageUrl ? <img src={imageUrl} alt="Image" /> : null;
};

export default useFetch;
