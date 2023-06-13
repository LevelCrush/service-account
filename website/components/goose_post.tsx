import { useEffect, useState } from 'react';

function renderGoosePost(props: React.PropsWithChildren, gooseRoll: number) {
  console.log('Goose Roll: ', gooseRoll);
  switch (gooseRoll) {
    case 1:
      return (
        <p>
          10 glimmer mithrix gets his shit kicked in by eramis as eido watches,
          then as her forces begin to overwhelm us and whats left of house
          light, shaw han comes in, says some cayde like shit, throws a trip
          mine at eramis. Completely whiffs the throw, shoots a gally rocket,
          somehow misses near point blank and hits the tripmine instead, blowing
          eramis into tomorrow and thats the rest of the seasonal story. Eido,
          empowered by watching the 3rd greatest guardian shes ever seen (Us,
          possibly saint, than SHAW HAN), picks up her fathers splicer gauntlet.
          Imbued now with the power of the darkness relics shes studying and the
          splicer gauntlet, begins to channel a green, slimeish color along her
          body. She turned towards the House Salvation ketch, rage bright and
          alight within all four of her eyes. her bottom 2 hands starting to
          grow green claws out of darkness, her top hands materializing a whip
          in one, and a yoyo in the other (screw warlocks). As she started
          grappling at speeds almost faster than a stasis titan with antaeus
          wards, a lone ship appears right above it. Ikora blinks out of the
          ship, throws 6 nova bombs at the fleeing ketch, followed by 9 minutes
          of chaos reach, destroying all traces of it. (edited) September 17,
          2022
        </p>
      );
    case 2:
      return (
        <img
          src="/goosepost_1.jpg"
          className="w-full max-w-[20rem] block mx-auto"
        />
      );
    case 3:
      return (
        <img
          src="/goosepost_2.jpg"
          className="w-full max-w-[20rem] block mx-auto "
        />
      );
    case 4:
      return (
        <img
          src="/goosepost_3.jpg"
          className="w-full max-w-[20rem] block mx-auto"
        />
      );
    default:
      return props.children;
  }
}

export const GoosePost = (props: React.PropsWithChildren) => {
  const [gooseRoll, setGooseRoll] = useState(0);
  const gooseTypes = 4;
  useEffect(() => {
    setGooseRoll(Math.floor(Math.random() * gooseTypes) + 1);
  }, []);

  return (
    <div className="goose-post w-full">{renderGoosePost(props, gooseRoll)}</div>
  );
};

export default GoosePost;
