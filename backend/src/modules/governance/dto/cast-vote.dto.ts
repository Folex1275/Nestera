import { IsString, IsEnum, IsNumber, Min } from 'class-validator';
import { ApiProperty } from '@nestjs/swagger';
import { VoteDirection } from '../entities/vote.entity';

export class CastVoteDto {
  @ApiProperty({ description: 'Voter wallet address', example: '0xabcd...' })
  @IsString()
  walletAddress: string;

  @ApiProperty({ enum: VoteDirection, example: VoteDirection.FOR })
  @IsEnum(VoteDirection)
  direction: VoteDirection;

  @ApiProperty({
    description: 'Vote weight (voting power)',
    example: 100.5,
    minimum: 0,
  })
  @IsNumber()
  @Min(0)
  weight: number;
}
