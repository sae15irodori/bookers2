class UsersController < ApplicationController
  def show
    @user = User.find(params[:id])#特定ﾕｰｻﾞｰidのﾚｺｰﾄﾞ取得
    @books = @user.books
  end

  def index
    @users = User.all
  end

  def edit
    @user = User.find(params[:id])
  end

  def update
    @user = User.find(params[:id])
    @user.update(user_params)
    redirect_to users_path(@user.id)
  end


    private
    def user_params
      params.require(:user).permit(:name, :introduction)
    end

end
