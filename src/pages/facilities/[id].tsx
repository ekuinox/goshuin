import Link from 'next/link';
import { ParsedUrlQuery } from "querystring";
import { Grid, Typography } from '@mui/material';
import { GetStaticPaths, GetStaticProps } from "next";
import { Facility } from "../../../lib/facility";

export interface Props {
    facility: Facility;
}

export interface Params extends ParsedUrlQuery {
    id: string;
}

export const getStaticPaths: GetStaticPaths<Params> = async () => {
    const { getFacilityIds } = await import('../../../lib/facility');
    const ids = await getFacilityIds();
    const paths = ids.map((id) => ({ params: { id } }));
    return {
        paths,
        fallback: false,
    };
};

export const getStaticProps: GetStaticProps<Props, Params> = async ({ params }) => {
    const { getFacility } = await import('../../../lib/facility');
    const facility = await getFacility(params!.id);
    return {
        props: {
            facility,
        },
    };
};

const IMAGE_WIDTH = '300px';

export const FacilityView = ({ facility }: Props) => {
    return (
        <Grid
            container
            direction='column'
        >
            <Grid item>
                <Typography variant='h3'>
                    {facility.name}
                </Typography>
            </Grid>
            <Grid item>
                <Link href={`https://www.google.com/maps/search/${facility.coordinate.lat},${facility.coordinate.lon}`}>
                    Google Maps
                </Link>
            </Grid>
            {facility.memo && (
                <Grid item>
                    <Typography variant='body1'>
                        {facility.memo}
                    </Typography>
                </Grid>
            )}
            {facility.goshuinList.map((goshuin, i) => (
                <Grid
                    key={i}
                    container
                    direction='column'
                >
                    <Grid item>
                        {new Date(goshuin.date).toLocaleDateString()}
                    </Grid>
                    {goshuin.description && (
                        <Grid item>
                            {goshuin.description}
                        </Grid>
                    )}
                    <Grid
                        container
                        direction='row'
                        spacing={2}
                    >
                        {goshuin.pictureUrls.map((url) => (
                            <Grid
                                item
                                key={url}
                            >
                                <img
                                    width={IMAGE_WIDTH}
                                    src={url}
                                />
                            </Grid>
                        ))}
                    </Grid>
                </Grid>
            ))}
            {facility.attachments && (
                <Grid
                    container
                    direction='row'
                    spacing={2}
                >
                    {facility.attachments.map((attachment, i) => (
                        <Grid item key={attachment.mediaUrl} >
                            <img
                                width={IMAGE_WIDTH}
                                src={attachment.mediaUrl}
                            />
                        </Grid>
                    ))}
                </Grid>
            )}
        </Grid>
    );
};

export default FacilityView;
