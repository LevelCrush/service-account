import DestinySearchBar from '@components/destiny_search_bar';
import Container from '@components/elements/container';

export const HomePage = () => (
  <Container>
    <h2>Big Destiny Search</h2>
    <DestinySearchBar className="max-w-[40rem] w-full mt-4  mx-auto" />
  </Container>
);
export default HomePage;
