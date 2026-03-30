import {
  Controller,
  Post,
  Param,
  UseGuards,
  HttpCode,
  HttpStatus,
} from '@nestjs/common';
import {
  ApiTags,
  ApiBearerAuth,
  ApiParam,
  ApiOperation,
} from '@nestjs/swagger';
import { JwtAuthGuard } from '../../auth/guards/jwt-auth.guard';
import { CurrentUser } from '../../common/decorators/current-user.decorator';
import { WaitlistService } from './waitlist.service';

@ApiTags('savings')
@Controller('savings/products')
export class WaitlistController {
  constructor(private readonly waitlistService: WaitlistService) {}

  @Post(':id/waitlist')
  @UseGuards(JwtAuthGuard)
  @HttpCode(HttpStatus.CREATED)
  @ApiBearerAuth()
  @ApiParam({ name: 'id', description: 'Product UUID' })
  @ApiOperation({ summary: 'Join waitlist for a savings product' })
  async join(
    @Param('id') id: string,
    @CurrentUser() user: { id: string; email: string },
  ) {
    const { entry, position } = await this.waitlistService.joinWaitlist(
      user.id,
      id,
    );

    return { id: entry.id, position };
  }
}
